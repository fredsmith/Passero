import type { PasswordEntry } from "./types";

export function filterTree(
  entries: PasswordEntry[],
  query: string,
): PasswordEntry[] {
  if (!query.trim()) return entries;
  const lower = query.toLowerCase();

  return entries
    .map((entry) => {
      if (entry.is_dir) {
        const filteredChildren = filterTree(entry.children, query);
        if (filteredChildren.length > 0) {
          return { ...entry, children: filteredChildren };
        }
        if (entry.name.toLowerCase().includes(lower)) {
          return entry;
        }
        return null;
      }
      if (entry.name.toLowerCase().includes(lower) || entry.path.toLowerCase().includes(lower)) {
        return entry;
      }
      return null;
    })
    .filter((e): e is PasswordEntry => e !== null);
}

export function parsePasswordContent(content: string): {
  password: string;
  metadata: Record<string, string>;
  otpauthUri: string | null;
} {
  const lines = content.split("\n");
  const password = lines[0] || "";
  const metadata: Record<string, string> = {};
  let otpauthUri: string | null = null;
  let currentKey: string | null = null;

  for (let i = 1; i < lines.length; i++) {
    const line = lines[i];
    const trimmed = line.trim();
    if (!trimmed) {
      currentKey = null;
      continue;
    }

    // Detect otpauth:// URIs on their own line
    if (trimmed.startsWith("otpauth://")) {
      otpauthUri = trimmed;
      currentKey = null;
      continue;
    }

    // YAML multi-line continuation: indented lines after a "key: |" value
    if (currentKey && (line.startsWith("  ") || line.startsWith("\t"))) {
      const existing = metadata[currentKey];
      metadata[currentKey] = existing + "\n" + trimmed;
      continue;
    }

    // Standard key: value pair
    const colonIdx = trimmed.indexOf(":");
    if (colonIdx > 0) {
      const key = trimmed.substring(0, colonIdx).trim();
      const value = trimmed.substring(colonIdx + 1).trim();
      // Check if the value contains an otpauth URI (e.g., "otp: otpauth://totp/...")
      const otpMatch = value.match(/(otpauth:\/\/\S+)/);
      if (otpMatch) {
        otpauthUri = otpMatch[1];
        currentKey = null;
      } else if (value === "|") {
        // YAML block scalar — start collecting continuation lines
        metadata[key] = "";
        currentKey = key;
      } else {
        metadata[key] = value;
        currentKey = null;
      }
    } else {
      currentKey = null;
    }
  }

  return { password, metadata, otpauthUri };
}

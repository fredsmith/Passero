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
} {
  const lines = content.split("\n");
  const password = lines[0] || "";
  const metadata: Record<string, string> = {};

  for (let i = 1; i < lines.length; i++) {
    const line = lines[i].trim();
    if (!line) continue;
    const colonIdx = line.indexOf(":");
    if (colonIdx > 0) {
      const key = line.substring(0, colonIdx).trim();
      const value = line.substring(colonIdx + 1).trim();
      metadata[key] = value;
    }
  }

  return { password, metadata };
}

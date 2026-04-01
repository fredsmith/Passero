export interface PasswordEntry {
  path: string;
  name: string;
  is_dir: boolean;
  children: PasswordEntry[];
}

export interface AppConfig {
  pass_binary: string | null;
  gpg_binary: string | null;
  git_binary: string | null;
  password_store_dir: string | null;
  clipboard_timeout: number;
}

export interface GpgKey {
  id: string;
  fingerprint: string;
  uid: string;
  trust: string;
}

export interface GitLogEntry {
  hash: string;
  message: string;
  author: string;
  date: string;
}

export type View = "main" | "settings" | "generator";

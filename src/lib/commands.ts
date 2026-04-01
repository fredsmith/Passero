import { invoke } from "@tauri-apps/api/core";
import type {
  PasswordEntry,
  AppConfig,
  GpgKey,
  GitLogEntry,
} from "./types";

export async function listPasswords(): Promise<PasswordEntry[]> {
  return invoke("list_passwords");
}

export async function showPassword(path: string): Promise<string> {
  return invoke("show_password", { path });
}

export async function insertPassword(
  path: string,
  content: string,
): Promise<void> {
  return invoke("insert_password", { path, content });
}

export async function editPassword(
  path: string,
  content: string,
): Promise<void> {
  return invoke("edit_password", { path, content });
}

export async function deletePassword(path: string): Promise<void> {
  return invoke("delete_password", { path });
}

export async function generatePassword(
  path: string,
  length: number,
  symbols: boolean,
): Promise<string> {
  return invoke("generate_password", { path, length, symbols });
}

export async function copyPassword(path: string): Promise<void> {
  return invoke("copy_password", { path });
}

export async function listGpgKeys(): Promise<GpgKey[]> {
  return invoke("list_gpg_keys");
}

export async function getStoreGpgId(): Promise<string> {
  return invoke("get_store_gpg_id");
}

export async function gitPull(): Promise<string> {
  return invoke("git_pull");
}

export async function gitPush(): Promise<string> {
  return invoke("git_push");
}

export async function gitLog(count?: number): Promise<GitLogEntry[]> {
  return invoke("git_log", { count });
}

export async function gitClone(
  url: string,
  path?: string,
): Promise<void> {
  return invoke("git_clone", { url, path });
}

export async function getConfig(): Promise<AppConfig> {
  return invoke("get_config");
}

export async function setConfig(config: AppConfig): Promise<void> {
  return invoke("set_config", { config });
}

export async function getPasswordStorePath(): Promise<string> {
  return invoke("get_password_store_path");
}

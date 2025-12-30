import { invoke } from '@tauri-apps/api/core'

export interface FileInfo {
  name: string;
  isDirectory: boolean;
  uri: string;
}

export async function requestPermission(): Promise<string> {
  return await invoke<{ uri: string }>('plugin:saf|request_permission').then(r => r.uri);
}

export async function checkPermission(uri: string): Promise<boolean> {
  return await invoke<{ granted: boolean }>('plugin:saf|check_permission', {
    payload: { uri }
  }).then(r => r.granted);
}

export async function listDir(uri: string): Promise<FileInfo[]> {
  return await invoke<{ files: FileInfo[] }>('plugin:saf|list_dir', {
    payload: { uri }
  }).then(r => r.files);
}

export async function readFile(uri: string): Promise<string> {
  return await invoke<{ content: string }>('plugin:saf|read_file', {
    payload: { uri }
  }).then(r => r.content);
}

export async function writeFile(uri: string, content: string): Promise<void> {
  return await invoke<void>('plugin:saf|write_file', {
    payload: { uri, content }
  });
}
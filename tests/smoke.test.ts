import { describe, it, expect } from 'vitest';
import fs from 'fs';
import path from 'path';

describe('Project Structure', () => {
  it('should have a src-tauri directory', () => {
    expect(fs.existsSync(path.resolve('src-tauri'))).toBe(true);
  });

  it('should have a tauri.conf.json file', () => {
    expect(fs.existsSync(path.resolve('src-tauri/tauri.conf.json'))).toBe(true);
  });
});

import { describe, it, expect, beforeEach } from 'vitest';
// @ts-ignore
import { ModManager } from '$lib/mods';

describe('ModManager', () => {
  let manager: any;

  beforeEach(() => {
    manager = new ModManager();
  });

  it('should initialize with an empty list', () => {
    expect(manager.getMods()).toEqual([]);
  });

  it('should add a mod', () => {
    manager.addMod({ name: 'Mod 1', enabled: true });
    expect(manager.getMods().length).toBe(1);
    expect(manager.getMods()[0].name).toBe('Mod 1');
  });

  it('should toggle a mod', () => {
    manager.addMod({ name: 'Mod 1', enabled: false });
    manager.toggleMod('Mod 1');
    expect(manager.getMods()[0].enabled).toBe(true);
  });

  it('should reorder mods', () => {
    manager.addMod({ name: 'Mod 1', enabled: true });
    manager.addMod({ name: 'Mod 2', enabled: true });
    manager.reorder(0, 1);
    expect(manager.getMods()[0].name).toBe('Mod 2');
    expect(manager.getMods()[1].name).toBe('Mod 1');
  });
});

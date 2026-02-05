export interface Mod {
    name: string;
    enabled: boolean;
}

export class ModManager {
    private mods: Mod[] = [];

    getMods(): Mod[] {
        return this.mods;
    }

    addMod(mod: Mod) {
        this.mods.push(mod);
    }

    toggleMod(name: string) {
        const mod = this.mods.find(m => m.name === name);
        if (mod) {
            mod.enabled = !mod.enabled;
        }
    }

    reorder(fromIndex: number, toIndex: number) {
        const [moved] = this.mods.splice(fromIndex, 1);
        this.mods.splice(toIndex, 0, moved);
    }
}

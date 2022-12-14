import { Theme } from "@emotion/react";
import { invoke } from "@tauri-apps/api";

export async function settingThemeChange(key: string, value: Theme): Promise<void> {
  await invoke("setting_theme_change", { key, value });
}

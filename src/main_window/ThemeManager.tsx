import { ThemeProvider } from "@emotion/react";
import { listen } from "@tauri-apps/api/event";
import { createContext, FC, ReactNode, useEffect, useState } from "react";

import { settingThemeGetActivated, settingThemeGetAll } from "../commands/setting";
import { carbon } from "./theme/carbon";

interface Props {
  children: ReactNode;
}
type UnlistenFn = () => void;

export const ThemeManager: FC<Props> = ({ children }) => {
  const [themeName, setThemeName] = useState("carbon");
  const [theme, setTheme] = useState(carbon);
  const ThemeContext = createContext({ themeName, setThemeName });

  useEffect(() => {
    // * Consume theme_activated event.
    interface Payload {
      activatedTheme: string;
    }
    const unlistenPromise = listen<Payload>("theme_activated", ({ payload }) => {
      setThemeName(payload.activatedTheme);
    });

    return () => {
      void unlistenPromise.then((unlisten: UnlistenFn) => {
        unlisten();
      });
    };
  }, []);

  useEffect(() => {
    // * Get activated theme.
    void settingThemeGetActivated().then((activateTheme) => {
      setThemeName(activateTheme);
    });
  }, []);

  useEffect(() => {
    // * Get theme obj associated to the themeName.
    void settingThemeGetAll().then((res) => {
      setTheme(res[themeName]);
    });
  }, [themeName]);

  return (
    <ThemeContext.Provider value={{ themeName, setThemeName }}>
      <ThemeProvider theme={theme}>{children}</ThemeProvider>
    </ThemeContext.Provider>
  );
};

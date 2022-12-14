import { useToast } from "@chakra-ui/react";
import { FC, useState } from "react";

import { generateIndex } from "../../../../../commands/plugin/appsearch/generateIndex";
import { SettingItem, SettingItemButton } from "../../../parts/main";

export const Generate: FC = () => {
  const [isLoading, setIsLoading] = useState(false);
  const errorToast = useToast();

  async function handleClick(): Promise<void> {
    setIsLoading(true);
    generateIndex(false)
      .then((s) => {
        setIsLoading(false);
      })
      .catch((e) => {
        errorToast({
          title: "ERROR",
          description: "Failed to generate indexes in application table.",
          position: "top",
          status: "error",
          isClosable: true,
          duration: 5000
        });
        setIsLoading(false);
      });
  }

  return (
    <>
      <SettingItem title="Generate" description="Generate index in application table.">
        {isLoading ? (
          <SettingItemButton isLoading loadingText="Generating" onClick={handleClick}>
            Generate
          </SettingItemButton>
        ) : (
          <SettingItemButton onClick={handleClick}>Generate</SettingItemButton>
        )}
      </SettingItem>
    </>
  );
};

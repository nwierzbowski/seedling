import { useEffect, useState } from "react";
import { useKeyPress } from "@xyflow/react";

/**
 * Custom hook for handling Shift+A keyboard shortcut
 * @returns Array with [isShiftAPressed, setShowMenu]
 */
export const useShiftAHotkey = () => {
  // Use the useKeyPress hook from React Flow to detect Shift+A
  const isShiftAPressed = useKeyPress(['Shift', 'A']);

  const [showMenu, setShowMenu] = useState(false);

  // Toggle menu when shift+A is pressed
  useEffect(() => {
    if (isShiftAPressed) {
      setShowMenu(prev => !prev);
    }
  }, [isShiftAPressed]);

  return [showMenu, setShowMenu] as const;
};
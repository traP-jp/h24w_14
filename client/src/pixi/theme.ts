// @ts-expect-error tailwind config
import tailwindConfig from "../../tailwind.config";

const { colors } = tailwindConfig.theme.extend;

type ThemeColors = Record<
  | "backgroundPrimary"
  | "backgroundSecondary"
  | "backgroundTertiary"
  | "textPrimary"
  | "textSecondary"
  | "textTertiary"
  | "accentPrimary"
  | "accentSecondary"
  | "accentTertiary"
  | "accentHover",
  string
>;

export const themeColors: ThemeColors = {
  backgroundPrimary: colors["background-primary"],
  backgroundSecondary: colors["background-secondary"],
  backgroundTertiary: colors["background-tertiary"],
  textPrimary: colors["text-primary"],
  textSecondary: colors["text-secondary"],
  textTertiary: colors["text-tertiary"],
  accentPrimary: colors["accent-primary"],
  accentSecondary: colors["accent-secondary"],
  accentTertiary: colors["accent-tertiary"],
  accentHover: colors["accent-hover"],
};

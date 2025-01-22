/** @type {import('tailwindcss').Config} */
export default {
  content: ["./index.html", "./src/**/*.{js,ts,jsx,tsx}"],
  theme: {
    extend: {
      colors: {
        "background-primary": "#FFFFFF",
        "background-secondary": "#F5F5F5",
        "background-tertiary": "#D9D9D9",
        "text-primary": "#1E1E1E",
        "text-secondary": "#757575",
        "text-tertiary": "#B3B3B3",
        "accent-primary": "#a0d911",
        "accent-secondary": "#bae637",
        "accent-tertiary": "#d3f261",
        "accent-hover": "#7cb305",
      },
    },
  },
  plugins: [],
};

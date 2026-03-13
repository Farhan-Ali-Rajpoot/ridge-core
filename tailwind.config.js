
const config = {
  content: [
    "./src/**/*.rs",
  ],
  darkMode: ['selector', '[data-theme=dark]'],
  theme: {
    extend: {
      fontFamily: {
        brisa: ["var(--font-brisa)", "serif"],
        "haffer-montreal": ["var(--font-haffer-montreal)", "sans-serif"],
      },
      keyframes: {
        // This handles the growing/shrinking of the line
        "google-dash": {
          "0%": { strokeDasharray: "1, 150", strokeDashoffset: "0" },
          "50%": { strokeDasharray: "90, 150", strokeDashoffset: "-35" },
          "100%": { strokeDasharray: "90, 150", strokeDashoffset: "-124" },
        },
      },
      animation: {
        // We combine the standard spin with our custom dash animation
        "google-dash": "google-dash 1.5s ease-in-out infinite",
      },
    },
  },
  plugins: [
    require("tailwind-scrollbar"),
    function ({ addUtilities }) {
      const newUtilities = {
        ".no-transition": {
          "transition": "none !important",
          "& *": {
            transition: "none !important",
          },
          "& *:before": {
            transition: "none !important",
          },
          "& *:after": {
            transition: "none !important",
          },
        },
      };
      addUtilities(newUtilities);
    },
  ],
};

export default config;

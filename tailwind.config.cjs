module.exports = {
  content: ["./src/**/*.{html,js,svelte,ts}"],

  // REMINDER: theme can be generated from daisyui.
  // Copy-paste theme colors into tailwind colors
  // section
  theme: {
    extend: {
      colors: {
        primary: "#111127",
        secondary: "#6d28d9",
        accent: "#4338ca",
        neutral: "#111127",
        "base-100": "#111127",
        info: "#3ABFF8",
        success: "#36D399",
        warning: "#FBBD23",
        error: "#F87272",
      },
    },
  },

  plugins: [require("@tailwindcss/typography"), require("@tailwindcss/forms")],
};

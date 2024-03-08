/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./src/**/*.{html,js,svelte,ts}",
  ],

  theme: {
    extend: {},
  },

  // plugins: [],
  plugins: [require("daisyui")],

  // temp darkmode stuff here
  darkMode: "class",
};

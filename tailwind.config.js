/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./index.html", "./src/**/*.rs"],
  theme: {
    extend: {
      fontFamily: {
        fira: ["FiraCode", "monospace"],
      },
    },
  },
  plugins: [],
};

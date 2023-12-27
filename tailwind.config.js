/** @type {import('tailwindcss').Config} */

module.exports = {
  content: {
    relative: true,
    files: ["*.html", "./**/src/**/*.rs"],
  },
  theme: {
    extend: {
      fontFamily: {
        "content": ["Inter", "sans-serif"],
      },
      spacing: {
        "2/5": "40%",
      }
    },
  },
  plugins: [],
}

/** @type {import('tailwindcss').Config} */
module.exports = {
  content: { 
    files: ["*.html", "./src/**/*.rs"],
  },
  theme: {
    extend: {
        "colors": {
            "black": "#1b1b1b"
        }
    },
  },
  plugins: [],
}

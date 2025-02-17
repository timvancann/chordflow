/** @type {import('tailwindcss').Config} */
module.exports = {
  mode: "all",
  purge: ['./src/**/*.html', './src/**/*.rs', './src/**/*.js'],
  content: ["./src/**/*.{rs,html,css}", "./dist/**/*.html"],
  theme: {
    extend: {},
  },
  plugins: [],
};

/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["*.html", "app/**/*.rs", "node_modules/preline/dist/*.js"],
  plugins: [
    require("@catppuccin/tailwindcss")({
      prefix: "ctp",
      defaultFlavour: "latte",
    }),
    require("preline/plugin"),
  ],
};

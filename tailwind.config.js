/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["*.html", "app/**/*.rs"],
  plugins: [
    require("@catppuccin/tailwindcss")({
      prefix: "ctp",
      defaultFlavour: "latte",
    }),
  ],
};

module.exports = {
  // https://tailwindcss.com/docs/guides/vue-3-vite#configure-tailwind-to-remove-unused-styles-in-production
  purge: ["./index.html", "./src/**/*.{vue,js,ts,jsx,tsx}"],
  darkMode: false, // or 'media' or 'class'
  theme: {
    extend: {},
  },
  variants: {
    extend: {},
  },
  plugins: [],
};

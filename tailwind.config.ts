import type { Config } from "tailwindcss";

export default {
  theme: {
    extend: {
      colors: {
        customGreen: "#4CAF40",
        customOrange: "#FAA111",
        customGrey: "#D2C8C8",
      },
      fontFamily: {
        sans: ["Poppins", "sans-serif"],
      },
    },
  },
} satisfies Config;
import type { Config } from 'tailwindcss';
import flowbitePlugin from 'flowbite/plugin'

export default {
  content: ['./src/**/*.{html,js,svelte,ts}', './node_modules/flowbite-svelte/**/*.{html,js,svelte,ts}'],
  darkMode: 'class',
  theme: {
    extend: {
      colors: {
        // pumpkin
        primary: {
          '50': '#fff8ed',
          '100': '#fff0d5',
          '200': '#feddaa',
          '300': '#fec473',
          '400': '#fca03b',
          '500': '#fa8213',
          '600': '#eb680b',
          '700': '#c34e0b',
          '800': '#9b3e11',
          '900': '#7c3512',
          '950': '#431807',
        },
      }
    }
  },
  plugins: [flowbitePlugin]
} as Config;
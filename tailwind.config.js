/** @type {import('tailwindcss').Config} */
export default {
  content: ['./src/**/*.{html,js,svelte,ts}'],
  theme: {
    extend: {
      colors: {
        // Catppuccin Mocha Theme
        ctp: {
          // Base colors
          base: '#1e1e2e',
          mantle: '#181825',
          crust: '#11111b',
          
          // Surface colors
          surface0: '#313244',
          surface1: '#45475a',
          surface2: '#585b70',
          
          // Overlay colors
          overlay0: '#6c7086',
          overlay1: '#7f849c',
          overlay2: '#9399b2',
          
          // Text colors
          subtext0: '#a6adc8',
          subtext1: '#bac2de',
          text: '#cdd6f4',
          
          // Accent colors for monitoring
          lavender: '#b4befe',
          blue: '#89b4fa',      // CPU
          sapphire: '#74c7ec',
          sky: '#89dceb',
          teal: '#94e2d5',      // Network
          green: '#a6e3a1',     // Memory (healthy)
          yellow: '#f9e2af',    // Warning
          peach: '#fab387',
          maroon: '#eba0ac',
          red: '#f38ba8',       // Critical
          mauve: '#cba6f7',     // GPU
          pink: '#f5c2e7',
          flamingo: '#f2cdcd',
          rosewater: '#f5e0dc',
        },
      },
      fontFamily: {
        sans: ['Inter', 'ui-sans-serif', 'system-ui', 'sans-serif'],
        mono: ['JetBrains Mono', 'ui-monospace', 'monospace'],
      },
      backgroundImage: {
        'gradient-radial': 'radial-gradient(var(--tw-gradient-stops))',
        'glass': 'linear-gradient(rgba(49, 50, 68, 0.7), rgba(49, 50, 68, 0.3))',
      },
      backdropBlur: {
        'xs': '2px',
      },
      animation: {
        'fade-in': 'fadeIn 0.3s ease-in-out',
        'slide-in': 'slideIn 0.3s ease-out',
        'pulse-slow': 'pulse 3s cubic-bezier(0.4, 0, 0.6, 1) infinite',
      },
      keyframes: {
        fadeIn: {
          '0%': { opacity: '0' },
          '100%': { opacity: '1' },
        },
        slideIn: {
          '0%': { transform: 'translateY(-10px)', opacity: '0' },
          '100%': { transform: 'translateY(0)', opacity: '1' },
        },
      },
    },
  },
  plugins: [
    require('@tailwindcss/typography'),
  ],
}

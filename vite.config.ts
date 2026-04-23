import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';

export default defineConfig({
	plugins: [sveltekit()],
	optimizeDeps: {
		include: [
			'@creit.tech/stellar-wallets-kit',
			'@stellar/freighter-api',
			'@albedo-link/intent'
		]
	}
});

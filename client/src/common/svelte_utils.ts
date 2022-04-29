import { onDestroy } from 'svelte'

/**
 * setInterval wrapper
 * it automatically closes it on component destroy
 * 
 * Usage
 * 
 * ```
 * <script>
	  import { onInterval } from './common/svelte_utils.ts'

  	let counter = 0
  	onInterval(() => counter += 1, 1000)
  </script>
 * ```
 */
export function onInterval(callback: Function, milliseconds: number) {
	const interval = setInterval(callback, milliseconds)

	onDestroy(() => {
		clearInterval(interval)
	})
}

/**
 * Takes relative location, returns actual relative location
 */
export function getActualLocation(location: string): string {
	return `${window.location.pathname}${location}`
}

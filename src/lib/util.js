export const sleep = async (msTime) =>
	new Promise((res) => setTimeout(res, msTime));
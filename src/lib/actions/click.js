import { goto } from "$app/navigation";

/**
 *
 * @param {Element} e
 * @param {string|Function} action
 * @returns {{destroy: destroy}}
 */
const click = (e, action = '/') => {
	const href = e.getAttribute('href');
	e.removeAttribute('href');

	const fn =
		(!action || typeof action !== 'function') ?
			goto.bind(null, action || href) :
			action;
	const key = (evt) => {
		if (evt.key !== 'Enter') return;
		evt.preventDefault();
		fn(evt);
	};

	e.addEventListener('click', fn);
	e.addEventListener('keypress', key);

	return {
		destroy: () => {
			e.removeEventListener('click', fn);
			e.removeEventListener('keypress', key);
		},
	};
};

export default click;
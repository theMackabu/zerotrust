const urlRegex =
	'^(https?:\\/\\/)?((([a-z\\d]([a-z\\d-]*[a-z\\d])*)\\.)+[a-z]{2,}|((\\d{1,3}\\.){3}\\d{1,3}))(\\:\\d+)?(\\/[-a-z\\d%_.~+]*)*(\\?[;&a-z\\d%_.~+=-]*)?(\\#[-a-z\\d_]*)?$';

const validate = {
	len: (input: string, len: number) => input.length >= len,
	url: (input: string) => !!new RegExp(urlRegex, 'i').test(input),
	email: (input: string) => /^[a-zA-Z0-9/]+@[a-zA-Z0-9/]+\.[a-zA-Z0-9/]+$/.test(input),
	safe: (input: string, len: number) => /^[a-zA-Z0-9_\/\.-]*$/.test(input) && input.length >= len,
	display: (input: string, len: number) => /^[a-zA-Z0-9_\/\. -]*$/.test(input) && input.length >= len
};

const classNames = (...classes: Array<any>) => classes.filter(Boolean).join(' ');

export { validate, classNames };

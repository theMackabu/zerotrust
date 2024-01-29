import { useState, Fragment } from 'react';

const Error = (props: { error: { name: String; message: String } }) => {
	return (
		<Fragment>
			<p>name: {props.error.name}</p>
			<p>message: {props.error.message}</p>
		</Fragment>
	);
};

export default Error;

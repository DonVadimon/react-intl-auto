import {FormatXMLElementFn, Options as IntlMessageFormatOptions, PrimitiveType} from 'intl-messageformat';
import {MessageDescriptor as MessageDescriptorReactIntl} from 'react-intl';

export type MessageDescriptor = MessageDescriptorReactIntl | string;

declare module 'react-intl' {
	interface ExtractableMessage {
		[key: string]: MessageDescriptor;
	}

	interface Messages {
		[key: string]: MessageDescriptorReactIntl | string; // for babel-plugin-react-intl-auto
	}

	export function defineMessages<T extends ExtractableMessage>(
		messages: T,
	): {[K in keyof T]: MessageDescriptorReactIntl};

	interface IntlFormatters<T = any, R = T> {
		formatMessage(
			descriptor: MessageDescriptor,
			values?: Record<string, PrimitiveType | T | FormatXMLElementFn<T, R>>,
			opts?: IntlMessageFormatOptions,
		): string;
	}
}

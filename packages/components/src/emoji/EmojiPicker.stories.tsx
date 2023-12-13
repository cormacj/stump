import { Meta, StoryObj } from '@storybook/react'

import EmojiPicker from './EmojiPicker'

const StoryMeta: Meta<typeof EmojiPicker> = {
	component: EmojiPicker,
	title: 'emoji/EmojiPicker',
}
type Story = StoryObj<typeof EmojiPicker>

export const Default: Story = {
	args: {
		onEmojiSelect: console.debug,
		onLoadError: console.error,
	},
}

export default StoryMeta

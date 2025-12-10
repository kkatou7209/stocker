import type { Component } from 'solid-js';
import { useApp } from '@/app/contexts/AppContext';

const EntryJournalPage: Component = () => {
	const app = useApp();

	app.setPageTitle('記帳');

	return <article></article>;
};

export default EntryJournalPage;

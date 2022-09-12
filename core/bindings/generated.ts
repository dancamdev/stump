// DO NOT MODIFY THIS FILE, IT IS AUTOGENERATED

export interface User {
	id: string;
	username: string;
	role: string;
	userPreferences: UserPreferences | null;
}

export type UserRole = 'SERVER_OWNER' | 'MEMBER';

export interface UserPreferences {
	id: string;
	reduceAnimations: boolean;
	locale: string;
	libraryViewMode: string;
	seriesViewMode: string;
	collectionViewMode: string;
}

export interface LoginOrRegisterArgs {
	username: string;
	password: string;
}

export type FileStatus = 'UNKNOWN' | 'READY' | 'UNSUPPORTED' | 'ERROR' | 'MISSING';

export interface Library {
	id: string;
	name: string;
	description: string | null;
	path: string;
	status: string;
	updatedAt: string;
	series: Array<Series> | null;
	tags: Array<Tag> | null;
	libraryOptions: LibraryOptions;
}

export type LibraryScanMode = 'SYNC' | 'BATCHED' | 'NONE';

export interface LibraryOptions {
	id: string | null;
	convertRarToZip: boolean;
	hardDeleteConversions: boolean;
	createWebpThumbnails: boolean;
	libraryId: string | null;
}

export interface CreateLibraryArgs {
	name: string;
	path: string;
	description: string | null;
	tags: Array<Tag> | null;
	scanMode: LibraryScanMode | null;
	libraryOptions: LibraryOptions | null;
}

export interface UpdateLibraryArgs {
	id: string;
	name: string;
	path: string;
	description: string | null;
	tags: Array<Tag> | null;
	removedTags: Array<Tag> | null;
	libraryOptions: LibraryOptions;
	scanMode: LibraryScanMode | null;
}

export interface LibrariesStats {
	seriesCount: bigint;
	bookCount: bigint;
	totalBytes: bigint;
}

export interface Series {
	id: string;
	name: string;
	path: string;
	description: string | null;
	status: FileStatus;
	updatedAt: string;
	libraryId: string;
	library: Library | null;
	media: Array<Media> | null;
	mediaCount: number | null;
	tags: Array<Tag> | null;
}

export interface Media {
	id: string;
	name: string;
	description: string | null;
	size: number;
	extension: string;
	pages: number;
	updatedAt: string;
	checksum: string | null;
	path: string;
	status: FileStatus;
	seriesId: string;
	series: Series | null;
	readProgresses: Array<ReadProgress> | null;
	currentPage: number | null;
	tags: Array<Tag> | null;
}

export interface MediaMetadata {
	Series: string | null;
	Number: number | null;
	Web: string | null;
	Summary: string | null;
	Publisher: string | null;
	Genre: string | null;
	PageCount: number | null;
}

export interface ReadProgress {
	id: string;
	page: number;
	mediaId: string;
	media: Media | null;
	userId: string;
	user: User | null;
}

export interface Tag {
	id: string;
	name: string;
}

export type ViewMode = 'GRID' | 'LIST';

export interface Epub {
	mediaEntity: Media;
	spine: Array<string>;
	resources: Record<string, [string, string]>;
	toc: Array<EpubContent>;
	metadata: Record<string, Array<string>>;
	rootBase: string;
	rootFile: string;
	extraCss: Array<string>;
}

export interface EpubContent {
	label: string;
	content: string;
	playOrder: number;
}

export type JobStatus = 'RUNNING' | 'QUEUED' | 'COMPLETED' | 'CANCELLED' | 'FAILED';

export interface JobUpdate {
	runnerId: string;
	currentTask: bigint | null;
	taskCount: bigint;
	message: string | null;
	status: JobStatus | null;
}

export interface JobReport {
	id: string | null;
	kind: string;
	details: string | null;
	status: JobStatus;
	taskCount: number | null;
	completedTaskCount: number | null;
	secondsElapsed: bigint | null;
	completedAt: string | null;
}

export type CoreEvent =
	| { key: 'JobStarted'; data: JobUpdate }
	| { key: 'JobProgress'; data: JobUpdate }
	| { key: 'JobComplete'; data: string }
	| { key: 'JobFailed'; data: { runner_id: string; message: string } }
	| { key: 'CreateEntityFailed'; data: { runner_id: string | null; path: string; message: string } }
	| { key: 'CreatedMedia'; data: Media }
	| { key: 'CreatedMediaBatch'; data: bigint }
	| { key: 'CreatedSeries'; data: Series }
	| { key: 'CreatedSeriesBatch'; data: bigint };

export interface DirectoryListing {
	parent: string | null;
	files: Array<DirectoryListingFile>;
}

export interface DirectoryListingFile {
	isDirectory: boolean;
	name: string;
	path: string;
}

export interface DirectoryListingInput {
	path: string | null;
}

export interface Log {
	id: string;
	level: LogLevel;
	message: string;
	created_at: string;
	job_id: string | null;
}

export interface LogMetadata {
	path: string;
	size: bigint;
	modified: string;
}

export type LogLevel = 'ERROR' | 'WARN' | 'INFO' | 'DEBUG';

export type Direction = 'asc' | 'desc';

export interface PageParams {
	zeroBased: boolean;
	page: number;
	pageSize: number;
	orderBy: string;
	direction: Direction;
}

export interface PagedRequestParams {
	zeroBased: boolean | null;
	page: number | null;
	pageSize: number | null;
	orderBy: string | null;
	direction: Direction | null;
}

export interface PageInfo {
	totalPages: number;
	currentPage: number;
	pageSize: number;
	pageOffset: number;
	zeroBased: boolean;
}

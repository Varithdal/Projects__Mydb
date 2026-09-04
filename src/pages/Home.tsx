export default function Home() {
	return (
		<main>
			{ /* New Entry */ }
			<form>
				<label for="name">Name: </label>
				<input name="name" id="name" required />
				<label for="file-path"> File Path: </label>
				<input name="file-path" id="file-path" required />
				<br />

				<p>Media Type</p>
				<input type="radio" value="videos" name="media-type" id="media-type-videos" required />
				<label for="media-type-videos">Videos</label>
				<br />

				<p>File Type</p>
				<input type="radio" name="file-type" value="file-type-mp4" required />
				<label for="file-type-mp4">MP4</label>
				<br />
				<br />

				<button type="submit">Submit</button>
			</form>

			<hr />

			{ /* Retrive Entries */ }
			<form>
			</form>

		</main>
	);
}

import { normalizeFileName } from "./utils.js";

Array.from(document.getElementsByClassName('btn-file-prune')).forEach((btn) => {
  btn.addEventListener('click', async function () {

    const filename = this.dataset.key;
    const response = await fetch(`/prune-file/${encodeURIComponent(filename)}`, { method: 'DELETE' });

    if (response.status === 200) {
      this.closest('tr').remove();
      alert(`File ${filename} has been removed!`);
    } else {
      alert(`File ${filename} couldn't be removed!`);
      console.error('File removing failed:', await response.text());
    }
  });
});

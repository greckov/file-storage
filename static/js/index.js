const fileField = document.getElementById('inp-file-upload');
const btnUploadFile = document.getElementById('btn-file-upload');
const resultsTable = document.querySelector('table > tbody');


function setupPruneFileListener(button) {
  button.addEventListener('click', async function () {
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
}

Array.from(document.getElementsByClassName('btn-file-prune')).forEach(setupPruneFileListener);

fileField.addEventListener('change', function () {
  btnUploadFile.disabled = !Boolean(this.value);
});

btnUploadFile.addEventListener('click', async () => {
  const formData = new FormData();
  formData.append('file', fileField.files[0]);

  const response = await fetch('upload-file', {
    method: 'POST',
    body: formData
  });

  if (response.status === 201) {
    const filename = fileField.files[0].name;

    resultsTable.insertAdjacentHTML('beforeend', `
      <tr>
        <td>
          <a href="#">${filename}</a>
        </td>
        <td>
          <div class="btn-group">
            <a class="btn btn-outline-primary" download="download" href="${BUCKET_URL}${filename}">Download</a>
            <a class="btn btn-outline-danger btn-file-prune" data-key="${filename}">Prune</a>
          </div>
        </td>
      </tr>
    `);

    setupPruneFileListener(document.querySelector('table td:last-child .btn-file-prune'));
    fileField.value = '';

  } else {
    console.error(await response.text());
    alert("Failed to upload a file. Please, refresh page and try again");
  }
});

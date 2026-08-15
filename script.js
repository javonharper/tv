let currentSlot = null;

function renderSchedule() {
    if (typeof SCHEDULE === 'undefined') return;

    const now = new Date();
    const minutesSinceMidnight = now.getHours() * 60 + now.getMinutes();

    const grid = document.getElementById('grid');
    if (!grid) return;

    grid.innerHTML = '<div class="row header"><div></div><div>Now Playing</div></div>';

    for (const ch of SCHEDULE) {
        const nowPlaying = ch.programs.find(
            p => minutesSinceMidnight >= p.start && minutesSinceMidnight < p.end
        );

        const row = document.createElement('div');
        row.className = 'row';

        const nameCell = document.createElement('div');
        nameCell.textContent = ch.channel;

        const filmCell = document.createElement('div');
        if (nowPlaying) {
            if (nowPlaying.url) {
                const a = document.createElement('a');
                a.href = nowPlaying.url;
                a.target = '_blank';
                a.textContent = nowPlaying.title;
                filmCell.appendChild(a);
            } else {
                const span = document.createElement('span');
                span.className = 'movie-title';
                span.dataset.title = nowPlaying.title;
                span.textContent = nowPlaying.title;
                filmCell.appendChild(span);
            }
        } else {
            filmCell.textContent = 'No program currently playing.';
        }

        row.appendChild(nameCell);
        row.appendChild(filmCell);
        grid.appendChild(row);
    }

    activateWatchLinks();
}

function updateTime() {
    const now = new Date();
    const days = ['Sunday', 'Monday', 'Tuesday', 'Wednesday', 'Thursday', 'Friday', 'Saturday'];
    const months = ['January', 'February', 'March', 'April', 'May', 'June', 'July', 'August', 'September', 'October', 'November', 'December'];
    
    const dayName = days[now.getDay()];
    const monthName = months[now.getMonth()];
    const date = now.getDate();
    
    let hours = now.getHours();
    const minutes = now.getMinutes();
    const ampm = hours >= 12 ? 'pm' : 'am';
    hours = hours % 12 || 12;
    const minutesStr = minutes.toString().padStart(2, '0');
    
    document.getElementById('client-date').textContent = `${dayName} ${monthName} ${date}`;
    document.getElementById('client-time').textContent = `${hours}:${minutesStr}${ampm}`;

    // Re-render schedule if the 2-hour slot has changed
    const slot = Math.floor((now.getHours() * 60 + now.getMinutes()) / 120);
    if (slot !== currentSlot) {
        currentSlot = slot;
        renderSchedule();
    }
}

updateTime();
setInterval(updateTime, 1000);

// Hidden watch URL (base64 encoded) - activated by triple-clicking header
const ENCODED_URL = 'aHR0cHM6Ly9mbGl4dG9yLnRvL3Nob3cvc2VhcmNoL3tRVUVSWX0vZnJvbS8xODAwL3RvLzIwOTkvcmF0aW5nLzAvdm90ZXMvMC9sYW5ndWFnZS9hbGwvdHlwZS9hbGwvZ2VucmUvYWxsL3JlbGV2YW5jZS9wYWdlLzE=';

// Triple-click activation on header
let clickCount = 0;
let clickTimer = null;

document.querySelector('.heading')?.addEventListener('click', () => {
    clickCount++;
    
    if (clickCount === 3) {
        const isActive = localStorage.getItem('watch-active') === 'true';
        localStorage.setItem('watch-active', (!isActive).toString());
        clickCount = 0;
        activateWatchLinks();
    }
    
    clearTimeout(clickTimer);
    clickTimer = setTimeout(() => clickCount = 0, 500);
});

// Activate watch links if enabled
function activateWatchLinks() {
    const isActive = localStorage.getItem('watch-active') === 'true';
    const movieTitles = document.querySelectorAll('.movie-title');
    
    movieTitles.forEach(span => {
        const title = span.dataset.title;
        if (!title) return;
        
        if (isActive) {
            // Decode URL template and replace {QUERY}
            const urlTemplate = atob(ENCODED_URL);
            const query = title.replace(/ /g, '+');
            const url = urlTemplate.replace('{QUERY}', query);
            
            // Convert span to link
            const link = document.createElement('a');
            link.href = url;
            link.target = '_blank';
            link.textContent = title;
            link.className = 'movie-title';
            link.dataset.title = title;
            span.replaceWith(link);
        } else {
            // Convert link back to span if it's a link
            if (span.tagName === 'A') {
                const newSpan = document.createElement('span');
                newSpan.className = 'movie-title';
                newSpan.dataset.title = title;
                newSpan.textContent = title;
                span.replaceWith(newSpan);
            }
        }
    });
}

// Initialize on page load
activateWatchLinks();

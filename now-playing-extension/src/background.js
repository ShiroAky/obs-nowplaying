let lastTitle = "";
let lastArtist = "";

async function getNowPlayingFromTab(tabId) {
  try {
    const results = await chrome.scripting.executeScript({
      target: { tabId: tabId },
      func: () => {
        let title = "",
          artist = "",
          cover = "",
          duration = null;

        if (window.location.hostname.includes("music.youtube.com")) {
          title =
            document
              .querySelector(".title.ytmusic-player-bar")
              ?.textContent.trim() || "";
          artist =
            document
              .querySelector(
                "#layout > ytmusic-player-bar > div.middle-controls.style-scope.ytmusic-player-bar > div.content-info-wrapper.style-scope.ytmusic-player-bar > span > span.subtitle.style-scope.ytmusic-player-bar > yt-formatted-string"
              )
              ?.textContent.trim()
              .split("•")[0] || "";
          cover =
            document.querySelector("#song-image #thumbnail #img")?.src || "";
          // Tiempo actual en formato mm:ss
          duration =
            document
              .querySelector("#left-controls > span")
              ?.textContent.trim() || null;
        }

        return { title, artist, duration, cover };
      },
    });

    if (results?.[0]?.result) return results[0].result;
  } catch (e) {
    console.log("Error leyendo pestaña:", e);
  }
  return null;
}

async function encodeCover(url) {
  try {
    const res = await fetch(url);
    const blob = await res.blob();
    return await new Promise((resolve) => {
      const reader = new FileReader();
      reader.onloadend = () => resolve(reader.result.split(",")[1]);
      reader.readAsDataURL(blob);
    });
  } catch {
    return null;
  }
}

// Enviar solo título/portada/artista cuando cambia la canción
async function sendSongInfo(tabId, np) {
  let coverB64 = np.cover ? await encodeCover(np.cover) : null;

  fetch("http://127.0.0.1:7539/update", {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({
      title: np.title,
      artist: np.artist,
      duration: np.duration || null,
      cover: coverB64,
    }),
  })
    .then(() => {
      console.log("Nueva canción detectada:");
      console.log(`🎵 Título: ${np.title}`);
      console.log(`🎤 Artista: ${np.artist}`);
      console.log(`🖼 Portada: ${coverB64 ? "sí" : "no"}`);
    })
    .catch((err) => console.log("No se pudo enviar info:", err));
}

// Enviar solo duración/progreso cada segundo
async function sendDuration(tabId, duration) {
  fetch("http://127.0.0.1:7539/update", {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({
      title: lastTitle,
      artist: lastArtist,
      duration: duration,
      cover: null, // no reenviamos portada
    }),
  })
    .then(() => {
      console.log(`⏱ Progreso actualizado: ${duration}`);
    })
    .catch((err) => console.log("No se pudo enviar progreso:", err));
}

// Revisa pestañas cada segundo
setInterval(async () => {
  const tabs = await chrome.tabs.query({
    url: ["*://*.music.youtube.com/*"],
  });
  for (const tab of tabs) {
    if (!tab.id || !tab.url) continue;
    if (tab.url.startsWith("chrome")) continue;

    const np = await getNowPlayingFromTab(tab.id);
    if (!np || (!np.title && !np.artist)) continue;

    // Si cambia de canción -> enviamos toda la info
    if (np.title !== lastTitle || np.artist !== lastArtist) {
      lastTitle = np.title;
      lastArtist = np.artist;
      sendSongInfo(tab.id, np);
    } else {
      // Si sigue siendo la misma -> solo enviamos progreso
      if (np.duration) {
        sendDuration(tab.id, np.duration);
      }
    }
  }
}, 500);

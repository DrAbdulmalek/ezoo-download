<div dir="rtl">

<p align="center">
  <strong>إيزو داونلود — نسخة لينكس (مانجارو)</strong><br/>
  Linux / Manjaro port of <a href="https://github.com/AhmedEzo686/ezoo-download">AhmedEzo686/ezoo-download</a>
</p>

> 🌍 الأدلة الأخرى: [README.md](README.md) (الأصل بالإنجليزية) · هذا الملف (العربية)

---

## ما هذا؟

هذه **نسخة معدّلة من مدير التحميل Ezoo Download** لتعمل على **لينكس مانجارو** (وتعمل أيضًا على Arch وأغلب التوزيعات الشقيقة). المشروع الأصلي برنامج ويندوز مكتوب بـ **Rust + Slint** بُني بمساعدة الذكاء الاصطناعي (شاهد شرحه على قناة [Ezoo Tech على يوتيوب](https://youtu.be/kQ8tsjBcT7o))، وهنا تم تكييفه للينكس مع الحفاظ على إمكانية البناء على ويندوز دون تغيير.

## ما الذي تغيّر في هذه النسخة؟

| الميزة | على ويندوز (الأصل) | على مانجارو (هذه النسخة) |
|---|---|---|
| yt-dlp / ffmpeg / aria2c | تُنزَّل تلقائيًا كنسخ ويندوز إلى مجلد `tools` | تُؤخذ **من النظام** (PATH) — ثبّتها مرة واحدة عبر pacman والتطبيق يجدها تلقائيًا |
| تحديث الأدوات تلقائيًا | مدمج | عبر `sudo pacman -Syu` (مدير الحزم هو المسؤول) |
| التحديث الذاتي للبرنامج | مدمج (مثبّت Inno Setup) | معطّل — حدّث بإعادة البناء أو `makepkg` |
| البدء مع النظام | سجل ويندوز | **يعمل الآن** عبر XDG Autostart (`~/.config/autostart`) |
| أيقونة شريط المهام (Tray) | مدعومة | غير متاحة حاليًا (صمت مقصود — استخدم التصغير بدلًا منها) |
| أصوات الإشعارات | مدعومة | معطلة في هذه النسخة |
| تسجيل إضافة المتصفح تلقائيًا | عبر السجل | غير متاح — أضف الإضافة يدويًا بوضع المطوّر في فايرفوكس/كروم |

## 🚀 التثبيت على مانجارو

### الطريقة 1: سكربت البناء (الأسهل)

```bash
git clone https://github.com/DrAbdulmalek/ezoo-download.git
cd ezoo-download
./build-linux.sh --install-deps   # تثبيت الاعتماديات عبر pacman (يطلب كلمة السر)
./build-linux.sh --install        # بناء + تثبيت في ~/.local
```

بعد ذلك ستجد **Ezoo Download** في قائمة تطبيقات النظام.

### الطريقة 2: حزمة PKGBUILD (على طريقة آرش)

```bash
git clone https://github.com/DrAbdulmalek/ezoo-download.git
cd ezoo-download/packaging
makepkg -si
```

ستُثبَّت الحزمة في `/usr/bin/ezoo-download` مع قائمة التطبيقات والأيقونة، وتُدار عبر `pacman -R ezoo-download` للإزالة.

### الاعتماديات (إن أردت تثبيتها يدويًا)

```bash
# بناء
sudo pacman -S --needed base-devel rust openssl gtk3 libxkbcommon fontconfig wayland libglvnd
# أدوات مساعدة وقت التشغيل
sudo pacman -S --needed yt-dlp ffmpeg aria2
```

## 🛠️ البناء اليدوي (أي توزيعة)

```bash
cargo build --release --workspace
# الناتج: target/release/EzooDownload
```

ثم شغّله مباشرة أو ثبّته يدويًا مع `packaging/ezoo-download.desktop`.

## 📌 ملاحظات مهمة للمستخدم الجديد على لينكس

1. **أول تشغيل**: إذا نزّلت ملفًا من يوتيوب ولم يجد التطبيق `yt-dlp` أو `ffmpeg` ستظهر رسالة تحمل أمر التثبيت الصحيح — نفّذه وسيعمل كل شيء.
2. **التنزيلات** تُحفظ افتراضيًا في مجلد التنزيلات لديك، ويمكن تغييره من إعدادات البرنامج.
3. **قائمة "تحديث مُنزّل الفيديو"** في قائمة المساعدة تُخبرك بلطف على لينكس أن التحديث يتم عبر pacman — هذا سلوك مقصود.
4. **الإضافة للمتصفح**: مجلد `extension/` يحتوي إضافة التقاط الروابط؛ على لينكس حمّلها يدويًا من صفحة إضافات المتصفح بوضع المطوّر (لكن التطبيق يعمل كاملًا بدونها عبر لصق الروابط).
5. **KDE Plasma**: يعمل التشغيل التلقائي مباشرة. في **GNOME** يلزم امتداد [Background Agent](https://extensions.gnome.org) لدعم XDG Autostart للبرامج الخلفية (سلوك GNOME المعروف).

## 🔧 بنية التعديلات (للمطوّرين)

كل تعديلات النقل محصورة في `cfg!(windows)` / `#[cfg(not(windows))]` حتى لا تتأثر نسخة ويندوز إطلاقًا:

- `crates/engine/src/tools.rs` — تعطيل مُنزّل الأدوات الخاص بويندوز على لينكس
- `crates/engine/src/selfupdate.rs` — تعطيل التحديث الذاتي (لا مثبّت Inno Setup على لينكس)
- `crates/engine/src/{mux,torrent,ytdlp}.rs` — رسائل نقص الأدوات تقترح `pacman` على لينكس
- `crates/app/src/autostart.rs` — تنفيذ XDG Autostart كامل بدل السجل
- `build-linux.sh` + `packaging/` — البناء والتغليف

## 📄 الترخيص والشكر

المشروع الأصلي **Ezoo Download** من تطوير [AhmedEzo686](https://github.com/AhmedEzo686) (قناة [Ezoo Tech](https://youtu.be/kQ8tsjBcT7o) على يوتيوب) — شكرًا له على العمل الرائع وعلى كود أساسي نظيف جعل النقل للينكس ممكنًا. هذا النقل من إعداد [DrAbdulmalek](https://github.com/DrAbdulmalek) وفق رخصة المشروع [MIT](THIRD-PARTY-NOTICES.txt).

</div>

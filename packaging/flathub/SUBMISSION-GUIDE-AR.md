# تقديم Ezoo Download إلى Flathub — دليل التسليم اليدوي (مهم)

## لماذا يجب أن تُقدّم الطلب بنفسك (سياسة Flathub)

يمنع قسم [Generative AI policy](https://docs.flathub.org/docs/for-app-authors/requirements#generative-ai-policy)
في متطلبات Flathub صراحةً:

> «AI tools or agents must not open or automate Flathub submission pull requests,
> or generate their commit messages, descriptions, review comments, or replies.»
>
> «Flathub manifests must not contain AI-generated or AI-assisted content.»

أي: **يجب أن يفتح الإنسان طلب الدمج بنفسه ويكتب نصه بنفسه**، وأن يكون ملف الـ
manifest خاليًا من المحتوى المولَّد بالذكاء الاصطناعي. فُتح طلبان تجريبيان (#10854
و#10855) قبل اكتشاف هذه السياسة وأُغلقًا تلقائيًا بواسطة بوت Flathub — لا حاجة
لأي إجراء بشأنهما، ولا يُنصح بالتعليق عليهما.

لهذا السبب توقفت الأتمتة عند هذا الحد، وتتضمن هذه الوثيقة كل ما جهّز لتُكمله
أنت بنفسك في ساعة واحدة تقريبًا.

## ما جهّز لك مسبقًا (في هذا المستودع)

| الأصل | الموقع | ملاحظات |
|---|---|---|
| مرجع الـ manifest | `packaging/flatpak/com.ezoo.Download.yml` | مكتمل: ffmpeg 7.1.1 + aria2 1.37.0 + yt-dlp 2025.10.14 + Rust 1.99.1 مثبتة البصمات |
| metainfo كامل | `packaging/flatpak/com.ezoo.Download.metainfo.xml` | يتضمن لقطة شاشة حقيقية (عربي/إنجليزي) — متطلب Flathub الأساسي |
| ملف سطح المكتب | `packaging/flatpak/com.ezoo.Download.desktop` | يتضمن `X-Flatpak=com.ezoo.Download` |
| رخصة MIT | `LICENSE` | مزدوجة حقوق المؤلف الأصلي + منفذ لينكس |
| لقطة شاشة | `packaging/screenshots/main-window.png` | ملتقطة من بناء لينكس الحقيقي عبر Xvfb |
| فيديو عرض 20 ثانية | [ezoo-download-1.0.7-linux-demo.mp4](https://github.com/DrAbdulmalek/ezoo-download/releases/download/v1.0.7-linux/ezoo-download-1.0.7-linux-demo.mp4) | مرفق بأصل إصدار v1.0.7-linux — استخدمه في بند الفيديو أو سجّل نسختك من تشغيل الـ Flatpak (الأفضل) |

## خطواتك (كإنسان، بنفسك)

### 1) أعد كتابة الـ manifest بأسلوبك

اقرأ `packaging/flatpak/com.ezoo.Download.yml` سطرًا سطرًا وتأكد أنك تفهم كل
جزء، ثم **انسخه واكتبه بتحريرك الخاص** في ملف باسم `com.ezoo.Download.yml`.
نقطة وحيدة يجب تغييرها عن نسخة المستودع: وحدة التطبيق نفسها مصدرها محلي في
نسخة المستودع (`type: dir, path: ../..`)، بينما على Flathub يجب أن يكون مصدر
git مثبت الوسم:

```yaml
    sources:
      - type: git
        url: https://github.com/DrAbdulmalek/ezoo-download.git
        tag: v1.0.7-linux
        commit: e1140e42babe6b42da9b6bb4d7d20f3d62411677
        x-checker-data:
          type: git
          tag-pattern: ^v([\d.]+)-linux$
```

### 2) جهّز الفرع في الشوكة الجاهزة

يوجد لديك شوكة باسم `DrAbdulmalek/flathub`. من صفحتها على الويب:

1. أنشئ فرعًا جديدًا من `flathub:new-pr` باسم مثل `com.ezoo.Download`
   (الشوكة الحالية تحتوي الفرع `com.ezoo.Download` لكن التزاماته مولّدة
   آليًا — الأنظف إنشاء فرعك الخاص أو تعديل الملف بتحريرك).
2. ارفع ملف `com.ezoo.Download.yml` في **الجذر مباشرة** (top-level — ليس في
   مجلد فرعي؛ البوت يغلق أي طلب ملفاته داخل مجلدات برسالة «Files not in
   toplevel»).
3. اكتب رسالة الالتزام بنفسك.

### 3) افتح طلب الدمج ضد الفرع `new-pr`

⚠️ يجب أن يكون الـ base هو `new-pr` وليس `master` — البوت يغلق أي طلب ضد
`master` تلقائيًا.

### 4) املأ قالب القائمة الظاهر عند إنشاء الطلب، بصدق

ترجمة البنود للمساعدة (أكّدها كما تنطبق عليك):

- [ ] **وصف التطبيق** باختصار (جملتان تكفيان).
- [ ] **فيديو يعرض التطبيق على لينكس عبر الـ Flatpak** — أرفق رابط الفيديو
  (رابط أصل الإصدار أعلاه صالح، أو سجّل بنفسك `flatpak-builder` ثم شغّل
  `flatpak run com.ezoo.Download` وسجّل الشاشة — هذا أقوى أمام المراجعين).
- [ ] **معرّف التطبيق** يستوفي [اشتراطات المعرّف](https://docs.flathub.org/docs/for-app-authors/requirements#application-id).
  - ملاحظة: `com.ezoo.Download` يفترض ضبط نطاق `ezoo.com`. إن لا تملكه، المتعارف
    عليه معرّف قابل للتحقق عبر GitHub: `io.github.DrAbdulmalek.EzooDownload` —
    يمكن للمراجعين اقتراح التحويل إليه. جرّب أن تذكر في الطلب استعدادك للتحويل.
- [ ] قرأت واستوفيت [متطلبات التقديم](https://docs.flathub.org/docs/for-app-authors/requirements)
  و[دليل التقديم](https://docs.flathub.org/docs/for-app-authors/submission).
  - تاريخ تطوير ذو معنى: المستودع له تاريخ حقيقي (المستوى الأصلي + منفذ لينكس
    بإصدارات منشورة).
  - **الإفصاح عن مواد الذكاء الاصطناعي**: إن بقي أي محتوى مولَّد بالذكاء
    الاصطناعي في التطبيق أو التغليف فيجب إفصاحه بتحديد الأجزاء ونسبتها.
    الأفضل عمليًا: راجع وأعد صياغة كل ملف بنفسك قبل التقديم حتى يصبح الإفصاح
    «None» أو محدودًا بمواد واضحة.
  - **عدم استخدام أدوات/وكلاء ذكاء اصطناعي في فتح الطلب أو أتمتته أو مراسلات
    المراجعة** — بند يجب أن تجيبه بنفسك بصدق؛ لا تحقّه إن لم يكن صحيحًا.
- [ ] أنت **مؤلف/مطوّر/مساهم في المشروع**، أو تواصلت مع مطوّريه الأصليين
  (المستوى الأصلي: `AhmedEzo686/ezoo-download`) — أرفق رابط التواصل إن لم
  تكن منهم.

### 5) بعد الفتح

- يرصد البوت الطلبات الجديدة تلقائيًا؛ ويمكن تسريع الفحص بتعليق يحوي `/review`.
- إن أُغلق تلقائيًا فاقرأ «Diagnostics:» في التعليق: توب-ليفل للملفات، قائمة
  مكتملة، بند الفيديو مع رابط — الأسباب الثلاثة الأشهر.
- يمكن إعادة التحقق بتعديل نفس الطلب وتعليق `/review` (لا تفتح طلبات جديدة
  متتالية — البوت يصنّف ذلك سبامًا).

## بديل أسرع: توزيعات أخرى

إن أردت توزيعًا فوريًا أثناء انتظار مراجعة Flathub (قد تستغرق أسابيع): حزم
AUR جاهزة في `packaging/aur/` (انظر `packaging/aur/README-AR.md`)، وحزم
tar.gz لكل المعماريات على صفحة الإصدارات.

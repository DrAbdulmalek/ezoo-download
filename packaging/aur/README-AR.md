# نشر Ezoo Download على AUR — دليل خطوة بخطوة

هذا المجلد يحتوي على حزمتين جاهزتين للنشر على [AUR](https://aur.archlinux.org):

| الحزمة | الوصف | الجمهور |
|---|---|---|
| `ezoo-download-bin` | ثنائيات جاهزة من إصدار GitHub (أسرع، لا يتطلب تجميع) | **موصى بها لمعظم المستخدمين** |
| `ezoo-download` | بناء من الكود المصدري عبر `cargo` (يتطلب Rust ووقت تجميع أطول) | من يفضل البناء من المصدر |

> **ملاحظة مهمة:** نشر الحزمة يتطلب **حساب AUR** باسم المستخدم ولا يمكن أتمتته برمزيًا من هنا —
> الخطوات أدناه موجّهة لك لتشغيلها محليًا على أي جهاز به `makepkg` (أي توزيعة Arch/Manjaro/EndeavourOS).

## الخطوات

### 1) إنشاء حساب AUR وإضافة مفتاح SSH (لمرة واحدة)

1. سجّل حسابًا على <https://aur.archlinux.org/register>.
2. أضف مفتاح SSH العام من صفحة حسابك (My Account → SSH Public Key):
   ```bash
   ls ~/.ssh/id_ed25519.pub 2>/dev/null || ssh-keygen -t ed25519
   cat ~/.ssh/id_ed25519.pub
   ```

### 2) نشر حزمة `ezoo-download-bin`

```bash
# انسخ ملفات الحزمة من هذا المستودع إلى مكان عمل مؤقت
cp -r packaging/aur/ezoo-download-bin /tmp/
cd /tmp/ezoo-download-bin

# استنسخ مستودع AUR الفارغ (يُنشأ تلقائيًا عند أول دفع)
git clone ssh://aur@aur.archlinux.org/ezoo-download-bin.git aur-repo
cd aur-repo

# انسخ PKGBUILD و .SRCINFO داخله (احتفظ بملفات AUR الخاصة مثل .gitignore)
cp ../PKGBUILD ../.SRCINFO .

# تحقق محليًا قبل النشر (اختياري لكن مستحسن)
makepkg --printsrcinfo > .SRCINFO   # يعيد توليد .SRCINFO من PKGBUILD
makepkg -f                          # يبني الحزمة ويتحقق من المجاميع
namcap PKGBUILD                     # فحص الأخطاء إن كان namcap مثبتًا

# انشر
git add PKGBUILD .SRCINFO
git commit -m "ezoo-download-bin 1.0.7-1: initial import"
git push
```

### 3) نشر حزمة `ezoo-download` (بناء من المصدر)

كرّر نفس الخطوات مع مجلد `packaging/aur/ezoo-download` (اسم المستودع البعيد: `ezoo-download`).

### 4) التحقق بعد النشر

- صفحة الحزمة ستظهر خلال ثوانٍ: <https://aur.archlinux.org/packages/ezoo-download-bin>
- جرّبها على جهاز Arch: `yay -S ezoo-download-bin` أو `paru -S ezoo-download-bin`.
- راقب التعليقات وطلبات نك العلامة (out-of-date) من صفحة الحزمة.

## تحديث الحزمة لإصدار مستقبلي (مثال 1.0.8)

1. في `PKGBUILD`: غيّر `pkgver` وحدّث `sha256sums` — البصمات موجودة في ملفات
   `.sha256` المرفقة تلقائيًا مع كل إصدار على صفحة Releases.
2. أعِد توليد `.SRCINFO`: `makepkg --printsrcinfo > .SRCINFO`
3. `git commit && git push`.

## أتمتة التحديث المستقبلية (اختياري)

يمكنك لاحقًا إضافة سير عمل GitHub Actions يستخدم إجراءً مثل
[`KSXGitHub/github-actions-deploy-aur`](https://github.com/KSXGitHub/github-actions-deploy-aur)
مع `AUR_SSH_PRIVATE_KEY` كـ Secret في مستودع GitHub ليُحدّث AUR تلقائيًا مع كل وسم جديد.

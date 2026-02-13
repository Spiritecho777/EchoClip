using Avalonia;
using Avalonia.Controls;
using Avalonia.Input;
using Avalonia.Media.Imaging;
using Avalonia.Platform;
using Avalonia.Markup.Xaml.XamlIl.Runtime;
using Avalonia.Threading;
using EchoClip.Classe;
using System;
using System.Collections.ObjectModel;
using System.ComponentModel;
using System.Linq;
using System.Threading;
using System.Threading.Tasks;

namespace EchoClip
{
    public partial class MainWindow : Window
    {
        private string? lastclipboardText;
        private Timer? clipboardTimer;
        private bool _isPasting = false;

        public ObservableCollection<ClipboardItem> ClipboardItems { get; } = new();
        public MainWindow()
        {
            InitializeComponent();
            DataContext = this;
            Closing += OnClosing;
            ClipboardList.PointerPressed += OnClipboardItemClick;
            StartClipboardWatcher();
        }

        private void StartClipboardWatcher()
        {
            clipboardTimer = new Timer(_ =>
            {
                Dispatcher.UIThread.Post(async () =>
                {
                    var clipboard = TopLevel.GetTopLevel(this)?.Clipboard;
                    if (clipboard == null) return;
                    if (_isPasting) return;

                    var formats = await clipboard.GetFormatsAsync();

                    // IMAGE
                    var imageFormats = new[] { "image/png", "image/bmp", "image/jpeg", "PNG" };
                    var matchedFormat = formats.FirstOrDefault(f => imageFormats.Contains(f));

                    if (matchedFormat != null)
                    {
                        var data = await clipboard.GetDataAsync(matchedFormat);

                        Bitmap? bmp = null;

                        if (data is byte[] bytes)
                        {
                            using var ms = new System.IO.MemoryStream(bytes);
                            bmp = new Bitmap(ms);
                        }
                        else if (data is System.IO.Stream stream)
                        {
                            bmp = new Bitmap(stream);
                        }

                        if (bmp != null)
                        {
                            // Hash du contenu via les dimensions + pixels (approximatif mais fiable)
                            var hash = $"{bmp.PixelSize.Width}x{bmp.PixelSize.Height}";
                            if (!ClipboardItems.Any(i => i.ImageHashKey == hash))
                            {
                                ClipboardItems.Insert(0, new ClipboardItem
                                {
                                    ClipboardType = ClipboardType.Image,
                                    Image = bmp,
                                    ImageHashKey = hash,
                                    Timestamp = DateTime.Now
                                });

                                const int MaxImages = 100; 
                                var images = ClipboardItems.Where(i => i.ClipboardType == ClipboardType.Image).ToList(); 
                                if (images.Count > MaxImages) 
                                { 
                                    ClipboardItems.Remove(images.Last());
                                }
                            }
                        }
                        return;
                    }

                    // TEXTE / LIEN
                    var text = await clipboard.GetTextAsync();
                    if (!string.IsNullOrWhiteSpace(text))
                    {
                        var type = Uri.IsWellFormedUriString(text, UriKind.Absolute)
                            ? ClipboardType.Link
                            : ClipboardType.Text;

                        if (!ClipboardItems.Any(i => i.Content == text))
                        {
                            ClipboardItems.Insert(0, new ClipboardItem
                            {
                                ClipboardType = type,
                                Content = text,
                                Timestamp = DateTime.Now
                            });

                            const int MaxTexte = 100000;
                            var texte = ClipboardItems.Where(i => i.ClipboardType == ClipboardType.Text).ToList();
                            if (texte.Count > MaxTexte)
                            {
                                ClipboardItems.Remove(texte.Last());
                            }
                        }
                    }
                });

            }, null, TimeSpan.Zero, TimeSpan.FromMilliseconds(500));
        }


        private void OnClosing(object? sender, CancelEventArgs e)
        {
            e.Cancel = true;
            Hide();
        }

        private async void OnClipboardItemClick(object? sender, PointerPressedEventArgs e)
        {
            if (ClipboardList.SelectedItem is not ClipboardItem item)
                return;

            var point = e.GetCurrentPoint(this);
            if (!point.Properties.IsRightButtonPressed)
                return;

            if (item == null) return;

            var clipboard = TopLevel.GetTopLevel(this)?.Clipboard;
            if (clipboard == null) return;

            _isPasting = true;

            switch (item.ClipboardType)
            {
                case ClipboardType.Text:
                case ClipboardType.Link:
                    await clipboard.SetTextAsync(item.Content ?? string.Empty);
                    break;

                case ClipboardType.Image:
                    if (item.Image != null)
                        await SetClipboardImageAsync(item.Image);
                    break;
            }

            await Task.Delay(600);
            _isPasting = false;
        }

        private async Task SetClipboardImageAsync(Bitmap image)
        {
            try
            {
                using var memoryStream = new System.IO.MemoryStream();
                image.Save(memoryStream);
                memoryStream.Position = 0;

                var clipboard = TopLevel.GetTopLevel(this)?.Clipboard;
                if (clipboard == null) return;

                var data = new DataObject();

                data.Set("PNG", memoryStream.ToArray());

                await clipboard.SetDataObjectAsync(data);

                System.Diagnostics.Debug.WriteLine("Image copiée dans le clipboard");
            }
            catch (Exception ex)
            {
                System.Diagnostics.Debug.WriteLine($"Erreur: {ex.Message}");
            }
        }
    }
}
using Avalonia;
using Avalonia.Controls;
using Avalonia.Input;
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
            clipboardTimer = new Timer(async _ =>
            {
                var clipboard = TopLevel.GetTopLevel(this)?.Clipboard;
                if (clipboard == null) return;

                var text = await clipboard.GetTextAsync();
                if (!string.IsNullOrWhiteSpace(text) && !ClipboardItems.Any(i => i.Content == text))
                {
                    lastclipboardText = text;
                    Dispatcher.UIThread.Post(() =>
                    {
                        ClipboardItems.Insert(0, new ClipboardItem
                        {
                            Content = text,
                            Timestamp = DateTime.Now,
                            ClipboardType = ClipboardType.Text,
                        });
                    });
                }
            }, null, TimeSpan.Zero,TimeSpan.FromMilliseconds(500));
        }

        private void OnClosing(object? sender, CancelEventArgs e)
        {
            e.Cancel = true;
            Hide();
        }

        private async void OnClipboardItemClick(object? sender, PointerPressedEventArgs e)
        {
            if (ClipboardList.SelectedItem is ClipboardItem item)
            {
                var point = e.GetCurrentPoint(this);
                if (point.Properties.IsRightButtonPressed)
                {
                    var clipboard = TopLevel.GetTopLevel(this)?.Clipboard;
                    if (clipboard != null)
                        await clipboard.SetTextAsync(item.Content);
                }
            }
        }
    }
}


/*Fonctionnalité 
    fonction recherche
    export de la session ??
    Icône à côté = type de contenu ( texte,  image,  lien) ??
*/
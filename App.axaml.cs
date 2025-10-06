using EchoClip;
using Avalonia;
using Avalonia.Controls;
using Avalonia.Controls.ApplicationLifetimes;
using Avalonia.Markup.Xaml;
using System;

namespace EchoClip
{
    public partial class App : Application
    {
        public override void Initialize()
        {
            AvaloniaXamlLoader.Load(this);
        }

        public override void OnFrameworkInitializationCompleted()
        {
            if (ApplicationLifetime is IClassicDesktopStyleApplicationLifetime desktop)
            {
                var main = new MainWindow();
                main.Hide();

                var showItem = new NativeMenuItem("Afficher");
                showItem.Click += (_, _) =>
                {
                    if (main?.IsVisible != true)
                        main?.Show();
                    else
                        main?.Activate();
                };

                var quitItem = new NativeMenuItem("Quitter");
                quitItem.Click += (_, _) => Environment.Exit(0);

                var trayMenu = new NativeMenu();
                trayMenu.Items.Add(showItem);
                trayMenu.Items.Add( new NativeMenuItemSeparator());
                trayMenu.Items.Add(quitItem);

                var trayIcon = new TrayIcon
                {
                    Icon = new WindowIcon("Assets/icon.png"),
                    IsVisible = true,
                    Menu = trayMenu
                };
            }

            base.OnFrameworkInitializationCompleted();
        }
    }
}
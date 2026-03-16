using Avalonia.Platform;
using Avalonia.Media.Imaging;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace EchoClip.Classe
{
    public enum ClipboardType { Text,Image,Link}
    public class ClipboardItem : IDisposable
    {
        public string? Content { get; set; }
        public DateTime Timestamp { get; set; }
        public ClipboardType ClipboardType { get; set; }
        public Bitmap? Image { get; set; }
        public string? ImageHashKey { get; set; }

        public string Preview
        {
            get
            {
                if (ClipboardType == ClipboardType.Image) 
                    return "[Image]";

                if (string.IsNullOrEmpty(Content)) 
                    return ""; 

                return ClipboardType switch 
                { 
                    ClipboardType.Link => Content.Length > 80 ? Content[..80] + "..." : Content, 
                    ClipboardType.Text => Content.Length > 120 ? Content[..120] + "..." : Content, 
                    _ => Content 
                };

            }
        }

        public void Dispose()
        {
            Image?.Dispose();
            Image = null;
        }
    }
}

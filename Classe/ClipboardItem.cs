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
    public class ClipboardItem
    {
        public string? Content { get; set; }
        public DateTime Timestamp { get; set; }
        public ClipboardType ClipboardType { get; set; }
        public Bitmap? Image { get; set; }
        public string? ImageHashKey { get; set; }
    }
}

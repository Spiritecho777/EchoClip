using Avalonia.Data.Converters;
using EchoClip.Classe;
using System;
using System.Collections.Generic;
using System.Globalization;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace EchoClip.Converter
{
    public class ClipboardIconConverter : IValueConverter
    {
        public object Convert(object value, Type targetType, object parameter, CultureInfo culture)
        {
            return value switch
            {
                ClipboardType.Text => "📝",
                ClipboardType.Link => "🔗",
                ClipboardType.Image => "🖼️",
                _ => "❓"
            };
        }

        public object ConvertBack(object value,Type targetType, object parameter, CultureInfo culture) => throw new NotImplementedException();
    }
}

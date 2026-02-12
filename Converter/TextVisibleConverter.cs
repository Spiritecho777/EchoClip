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
    public class TextVisibleConverter : IValueConverter
    {
        public object Convert(object value, Type t, object p, CultureInfo c)
            => (ClipboardType)value != ClipboardType.Image;

        public object ConvertBack(object value, Type targetType, object parameter, CultureInfo culture) => throw new NotImplementedException();
    }
 
}

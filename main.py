import math
from pathlib import Path

from PIL import Image
from PIL import ImageCms
from PIL import ImageStat
from PIL.Image import Dither
from PIL.Image import Quantize
from Pylette import extract_colors
from Pylette import Palette
from tqdm import tqdm
from datetime import datetime

pic1 = Path('res/pic1.png')
pic2 = Path('res/pic2.jpg')
pic3 = Path('res/pic3.jpg')
pic4 = Path('res/pic4.jpg')

srgb_profile = ImageCms.createProfile("sRGB")
lab_profile = ImageCms.createProfile("LAB")


Color = (int, int, int)  # 3 channels


def calc_rgb_similarity(color_1: Color, color_2: Color) -> float:
    return math.sqrt(sum((
        (color_1[0] - color_2[0]) ** 2,
        (color_1[1] - color_2[1]) ** 2,
        (color_1[2] - color_2[2]) ** 2,
    )))


def calc_lab_similarity(color_1: Color, color_2: Color) -> float:
    return math.sqrt(sum((
        # skip L (luminance) channel
        (color_1[1] - color_2[1]) ** 2,
        (color_1[2] - color_2[2]) ** 2,
    )))


def look_for_replacement_color(palette: Palette, color: (int, int, int)) -> (int, int, int):
    # finds most similar color from given palette
    result = palette.colors[0]
    similarity = None

    for c in palette.colors:
        # print('color', c)
        # r = c.rgb[0]
        # g = c.rgb[1]
        # b = c.rgb[2]

        # calculated_similarity = calc_rgb_similarity(c.rgb, color)
        calculated_similarity = calc_lab_similarity(c.rgb, color)
        if similarity is None or calculated_similarity < similarity:
            result = (color[0], c.rgb[1], c.rgb[2])
            similarity = calculated_similarity

    # print(f'{color} replaced with {result.rgb}')
    return result


def convert_to_lab(img: Image) -> Image:
    rgb2lab_transform = ImageCms.buildTransformFromOpenProfiles(
        srgb_profile,
        lab_profile,
        "RGB",
        "LAB",
    )
    return ImageCms.applyTransform(img, rgb2lab_transform)
    # return ImageCms.profileToProfile(img, srgb_profile, lab_profile)


def extract_palette(img_path: Path) -> Palette:
    palette = extract_colors(img_path, palette_size=10, resize=False)
    print(palette)
    # palette.display(save_to_file=False)

    return palette


def main():
    im1 = Image.open(pic4)
    im1 = im1.convert('RGB')
    print(im1.mode)
    start_time = datetime.now()
    # im1 = im1.resize((512, 512))  # enhance performance
    # im1.quantize(colors=32).show()
    # im1.quantize(colors=32, dither=Dither.NONE).show()
    # im1.quantize(colors=16, method=Quantize.FASTOCTREE, dither=Dither.FLOYDSTEINBERG).show()
    # im1.quantize(colors=20, method=Quantize.FASTOCTREE, dither=Dither.NONE).show()
    # print(im1.mode, im1.getpixel((0, 0)))
    # im1 = convert_to_lab(im1)
    # print(im1.mode, im1.getpixel((0, 0)))
    # im1.show()

    # im2 = im1.quantize(colors=32)
    # im1.quantize(colors=8).show()
    im_size = im1.size
    # merged_image = Image.new('RGB', (2 * im_size[0], im_size[1]), (0, 0, 0))
    # merged_image.paste(im1, (0, 0))
    # merged_image.paste(im2, (im_size[0], 0))

    # merged_image.show()
    palette = extract_palette(pic1)
    # im1.point(lambda x: print("wtf", x))
    img_pixels = im1.load()
    for i in tqdm(range(im_size[0])):
        for j in range(im_size[1]):
            color = img_pixels[i, j]
            # print('color', color)
            img_pixels[i, j] = look_for_replacement_color(palette, color)
        # print()
    print('Done!')
    print('Time:', (datetime.now() - start_time).total_seconds(), 'sec')
    im1.show()
    im1.save('last_gen', format='png')


    stats = ImageStat.Stat(im1)
    print(f'mean {stats.mean}, median {stats.median}, var {stats.var},  stddev {stats.stddev}')
    # print(f'hist {im2.histogram()}')

#
# # snippet from github
# rgb_lab_transform = PIL.ImageCms.buildTransformFromOpenProfiles(PIL.ImageCms.createProfile('sRGB'), PIL.ImageCms.createProfile('LAB'), 'RGB', 'LAB')
# img = PIL.ImageCms.applyTransform(Image.open('filename.png').convert(mode='RGB'), rgb_lab_transform)
#
# # snippet from stackoverflow
# im2 = pyCMS.profileToProfile(im, pyCMS.createProfile("sRGB"), pyCMS.createProfile("LAB"))
#
# # another stackoverflow snippet
# from PIL import Image, ImageCms
# srgb_p = ImageCms.createProfile("sRGB")
# lab_p  = ImageCms.createProfile("LAB")
#
# rgb2lab = ImageCms.buildTransformFromOpenProfiles(srgb_p, lab_p, "RGB", "LAB")
# Lab = ImageCms.applyTransform(im, rgb2lab)

if __name__ == '__main__':
    main()

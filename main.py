from ctypes import cdll, c_char_p
import argparse
import os

DEBUG = False

# Load library
path_to_unsplash_wezterm = os.path.dirname(os.path.realpath(__file__))
path_to_lib = os.path.join(path_to_unsplash_wezterm, 'target',
                           'debug' if DEBUG else 'release', 'libunsplash_wezterm.so')
unsplash_wezterm = cdll.LoadLibrary(path_to_lib)

# Export functions
unsplash_wezterm.save_new_image.argtypes = [
    c_char_p,
    c_char_p,
    c_char_p,
    c_char_p,
    c_char_p,
    c_char_p,
    c_char_p,
    c_char_p
]
unsplash_wezterm.save_new_image.restype = c_char_p


class UnsplashParams:
    """
    class for UnsplashParams

    It contains parameters for random image API call.
    For more info, see: `https://unsplash.com/documentation#get-a-random-photo`

    Attributes
    ----------
    collections : str
    topics : str
    username : str
    query : str
    orientation : str
    content_filter : str
    """
    collections: str
    topics: str
    username: str
    query: str
    orientation: str
    content_filter: str

    def __init__(self,
                 collections: str = '',
                 topics: str = '',
                 username: str = '',
                 query: str = '',
                 orientation: str = '',
                 content_filter: str = '',
                 ):
        """
        Constructor for UnsplashParams

        Parameters
        ----------
        collections : str
        topics : str
        username : str
        query : str
        orientation : str
        content_filter : str
        """
        self.collections = collections
        self.topics = topics
        self.username = username
        self.query = query
        self.orientation = orientation
        self.content_filter = content_filter


def save_new_image(api_key: str, folder: str, params: UnsplashParams) -> str:
    """
    Save new image from Unsplash

    Parameters
    ----------
    api_key : str
    folder : str
    params : UnsplashParams
    """
    image_id: str = unsplash_wezterm.save_new_image(
        api_key.encode(),
        folder.encode(),
        params.collections.encode(),
        params.topics.encode(),
        params.username.encode(),
        params.query.encode(),
        params.orientation.encode(),
        params.content_filter.encode(),
    ).decode()
    if image_id == 'ERROR':
        raise Exception('Failed to get image from Unsplash')
    path = os.path.abspath(os.path.join(folder, image_id + '.jpg'))
    return path


# Setup CLI
parser = argparse.ArgumentParser(description='Get random images from Unsplash!')
parser.add_argument('--api_key', type=str,
                    default=os.getenv('UNSPLASH_API_KEY'),
                    help='Unsplash API key (env: UNSPLASH_API_KEY by default)')
parser.add_argument('--folder', type=str,
                    default=os.getcwd(), help='Folder to save images')
parser.add_argument('--collections', type=str,
                    default='', help='Collections')
parser.add_argument('--topics', type=str,
                    default='', help='Topics')
parser.add_argument('--username', type=str,
                    default='', help='Username')
parser.add_argument('--query', type=str,
                    default='', help='Query')
parser.add_argument('--orientation', type=str,
                    default='', help='Orientation')
parser.add_argument('--content_filter', type=str,
                    default='', help='Content filter')
args = parser.parse_args()

if __name__ == '__main__':
    params: UnsplashParams = UnsplashParams(
        collections=args.collections,
        topics=args.topics,
        username=args.username,
        query=args.query,
        orientation=args.orientation,
        content_filter=args.content_filter
    )
    try:
        path_to_image: str = save_new_image(args.api_key, args.folder, params)
        print(path_to_image)
    except Exception:
        print('ERROR')

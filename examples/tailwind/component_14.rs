use rust_web_ui::*;

pub struct Example14;

pub struct Example14Props {}

impl Default for Example14Props {
	fn default() -> Example14Props {
		Example14Props { }
	}
}

impl Widget<'_> for Example14 {
	type Props = Example14Props;

	fn render(mut ctx: Ctx<'_>, props: Example14Props) {
		ctx.styles(&[NoStyle::Noop("")]);
		ctx.set_attribute("style", "");
		
		ctx.child("1", Dynamic).run("div", |props| {
			props.styles(&[Style::Flex, Style::ItemsCenter, Style::JustifyCenter, NoStyle::Noop("bg-white"), Style::PaddingX(32), Style::PaddingY(48)]);
			
			props.child("1", Dynamic).run("div", |props| {
				props.styles(&[NoStyle::Noop("mx-auto"), NoStyle::Noop("w-full"), NoStyle::Noop("max-w-lg"), NoStyle::Noop("items-end"), NoStyle::Noop("justify-around"), Style::SpaceY(24), Screen::Small(&[Style::Flex]), Screen::Small(&[Style::SpaceY(0)])]);
				
				props.child("1", Dynamic).run("div", |props| {
					props.styles(&[Style::Flex, NoStyle::Noop("-space-x-1"), NoStyle::Noop("overflow-hidden")]);
					
					props.child("1", Dynamic).run("img", |props| {
						props.styles(&[Style::InlineBlock, Style::Width(24), Style::Width(24), NoStyle::Noop("rounded-full"), NoStyle::Noop("ring-2"), NoStyle::Noop("ring-white")]);
						props.set_attribute("src", "https://images.unsplash.com/photo-1491528323818-fdd1faba62cc?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80");
						props.set_attribute("alt", "");
					});
					props.child("3", Dynamic).run("img", |props| {
						props.styles(&[Style::InlineBlock, Style::Width(24), Style::Width(24), NoStyle::Noop("rounded-full"), NoStyle::Noop("ring-2"), NoStyle::Noop("ring-white")]);
						props.set_attribute("src", "https://images.unsplash.com/photo-1550525811-e5869dd03032?ixlib=rb-1.2.1&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80");
						props.set_attribute("alt", "");
					});
					props.child("5", Dynamic).run("img", |props| {
						props.styles(&[Style::InlineBlock, Style::Width(24), Style::Width(24), NoStyle::Noop("rounded-full"), NoStyle::Noop("ring-2"), NoStyle::Noop("ring-white")]);
						props.set_attribute("src", "https://images.unsplash.com/photo-1500648767791-00dcc994a43e?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2.25&w=256&h=256&q=80");
						props.set_attribute("alt", "");
					});
					props.child("7", Dynamic).run("img", |props| {
						props.styles(&[Style::InlineBlock, Style::Width(24), Style::Width(24), NoStyle::Noop("rounded-full"), NoStyle::Noop("ring-2"), NoStyle::Noop("ring-white")]);
						props.set_attribute("src", "https://images.unsplash.com/photo-1472099645785-5658abf4ff4e?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80");
						props.set_attribute("alt", "");
					});
				});
				props.child("3", Dynamic).run("div", |props| {
					props.styles(&[Style::Flex, NoStyle::Noop("-space-x-2"), NoStyle::Noop("overflow-hidden")]);
					
					props.child("1", Dynamic).run("img", |props| {
						props.styles(&[Style::InlineBlock, Style::Width(32), Style::Width(32), NoStyle::Noop("rounded-full"), NoStyle::Noop("ring-2"), NoStyle::Noop("ring-white")]);
						props.set_attribute("src", "https://images.unsplash.com/photo-1491528323818-fdd1faba62cc?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80");
						props.set_attribute("alt", "");
					});
					props.child("3", Dynamic).run("img", |props| {
						props.styles(&[Style::InlineBlock, Style::Width(32), Style::Width(32), NoStyle::Noop("rounded-full"), NoStyle::Noop("ring-2"), NoStyle::Noop("ring-white")]);
						props.set_attribute("src", "https://images.unsplash.com/photo-1550525811-e5869dd03032?ixlib=rb-1.2.1&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80");
						props.set_attribute("alt", "");
					});
					props.child("5", Dynamic).run("img", |props| {
						props.styles(&[Style::InlineBlock, Style::Width(32), Style::Width(32), NoStyle::Noop("rounded-full"), NoStyle::Noop("ring-2"), NoStyle::Noop("ring-white")]);
						props.set_attribute("src", "https://images.unsplash.com/photo-1500648767791-00dcc994a43e?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2.25&w=256&h=256&q=80");
						props.set_attribute("alt", "");
					});
					props.child("7", Dynamic).run("img", |props| {
						props.styles(&[Style::InlineBlock, Style::Width(32), Style::Width(32), NoStyle::Noop("rounded-full"), NoStyle::Noop("ring-2"), NoStyle::Noop("ring-white")]);
						props.set_attribute("src", "https://images.unsplash.com/photo-1472099645785-5658abf4ff4e?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80");
						props.set_attribute("alt", "");
					});
				});
				props.child("5", Dynamic).run("div", |props| {
					props.styles(&[Style::Flex, NoStyle::Noop("-space-x-2"), NoStyle::Noop("overflow-hidden")]);
					
					props.child("1", Dynamic).run("img", |props| {
						props.styles(&[Style::InlineBlock, Style::Width(40), Style::Width(40), NoStyle::Noop("rounded-full"), NoStyle::Noop("ring-2"), NoStyle::Noop("ring-white")]);
						props.set_attribute("src", "https://images.unsplash.com/photo-1491528323818-fdd1faba62cc?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80");
						props.set_attribute("alt", "");
					});
					props.child("3", Dynamic).run("img", |props| {
						props.styles(&[Style::InlineBlock, Style::Width(40), Style::Width(40), NoStyle::Noop("rounded-full"), NoStyle::Noop("ring-2"), NoStyle::Noop("ring-white")]);
						props.set_attribute("src", "https://images.unsplash.com/photo-1550525811-e5869dd03032?ixlib=rb-1.2.1&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80");
						props.set_attribute("alt", "");
					});
					props.child("5", Dynamic).run("img", |props| {
						props.styles(&[Style::InlineBlock, Style::Width(40), Style::Width(40), NoStyle::Noop("rounded-full"), NoStyle::Noop("ring-2"), NoStyle::Noop("ring-white")]);
						props.set_attribute("src", "https://images.unsplash.com/photo-1500648767791-00dcc994a43e?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2.25&w=256&h=256&q=80");
						props.set_attribute("alt", "");
					});
					props.child("7", Dynamic).run("img", |props| {
						props.styles(&[Style::InlineBlock, Style::Width(40), Style::Width(40), NoStyle::Noop("rounded-full"), NoStyle::Noop("ring-2"), NoStyle::Noop("ring-white")]);
						props.set_attribute("src", "https://images.unsplash.com/photo-1472099645785-5658abf4ff4e?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80");
						props.set_attribute("alt", "");
					});
				});
			});
		});
	}
}

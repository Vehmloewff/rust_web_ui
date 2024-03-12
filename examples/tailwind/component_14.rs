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
		ctx.styles(&[Style::Noop("")]);
		ctx.set_attribute("style", "");
		
		ctx.child("1", Dynamic).run("div", |mut props| {
			props.styles(&[Style::Flex, Style::ItemsCenter, Style::JustifyCenter, Style::Noop("bg-white"), Style::PaddingX(Size::Exact(32)), Style::PaddingY(Size::Exact(48))]);
			
			props.child("1", Dynamic).run("div", |mut props| {
				props.styles(&[Style::Noop("mx-auto"), Style::Width(Size::Full), Style::Noop("max-w-lg"), Style::Noop("items-end"), Style::Noop("justify-around"), Style::SpaceY(Size::Exact(24)), Style::OnScreen(Screen::Small, &[Style::Flex]), Style::OnScreen(Screen::Small, &[Style::SpaceY(Size::Exact(0))])]);
				
				props.child("1", Dynamic).run("div", |mut props| {
					props.styles(&[Style::Flex, Style::Noop("-space-x-1"), Style::Noop("overflow-hidden")]);
					
					props.child("1", Dynamic).run("img", |mut props| {
						props.styles(&[Style::InlineBlock, Style::Width(Size::Exact(24)), Style::Width(Size::Exact(24)), Style::Noop("rounded-full"), Style::Noop("ring-2"), Style::Noop("ring-white")]);
						props.set_attribute("src", "https://images.unsplash.com/photo-1491528323818-fdd1faba62cc?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80");
						props.set_attribute("alt", "");
					});
					props.child("3", Dynamic).run("img", |mut props| {
						props.styles(&[Style::InlineBlock, Style::Width(Size::Exact(24)), Style::Width(Size::Exact(24)), Style::Noop("rounded-full"), Style::Noop("ring-2"), Style::Noop("ring-white")]);
						props.set_attribute("src", "https://images.unsplash.com/photo-1550525811-e5869dd03032?ixlib=rb-1.2.1&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80");
						props.set_attribute("alt", "");
					});
					props.child("5", Dynamic).run("img", |mut props| {
						props.styles(&[Style::InlineBlock, Style::Width(Size::Exact(24)), Style::Width(Size::Exact(24)), Style::Noop("rounded-full"), Style::Noop("ring-2"), Style::Noop("ring-white")]);
						props.set_attribute("src", "https://images.unsplash.com/photo-1500648767791-00dcc994a43e?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2.25&w=256&h=256&q=80");
						props.set_attribute("alt", "");
					});
					props.child("7", Dynamic).run("img", |mut props| {
						props.styles(&[Style::InlineBlock, Style::Width(Size::Exact(24)), Style::Width(Size::Exact(24)), Style::Noop("rounded-full"), Style::Noop("ring-2"), Style::Noop("ring-white")]);
						props.set_attribute("src", "https://images.unsplash.com/photo-1472099645785-5658abf4ff4e?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80");
						props.set_attribute("alt", "");
					});
				});
				props.child("3", Dynamic).run("div", |mut props| {
					props.styles(&[Style::Flex, Style::Noop("-space-x-2"), Style::Noop("overflow-hidden")]);
					
					props.child("1", Dynamic).run("img", |mut props| {
						props.styles(&[Style::InlineBlock, Style::Width(Size::Exact(32)), Style::Width(Size::Exact(32)), Style::Noop("rounded-full"), Style::Noop("ring-2"), Style::Noop("ring-white")]);
						props.set_attribute("src", "https://images.unsplash.com/photo-1491528323818-fdd1faba62cc?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80");
						props.set_attribute("alt", "");
					});
					props.child("3", Dynamic).run("img", |mut props| {
						props.styles(&[Style::InlineBlock, Style::Width(Size::Exact(32)), Style::Width(Size::Exact(32)), Style::Noop("rounded-full"), Style::Noop("ring-2"), Style::Noop("ring-white")]);
						props.set_attribute("src", "https://images.unsplash.com/photo-1550525811-e5869dd03032?ixlib=rb-1.2.1&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80");
						props.set_attribute("alt", "");
					});
					props.child("5", Dynamic).run("img", |mut props| {
						props.styles(&[Style::InlineBlock, Style::Width(Size::Exact(32)), Style::Width(Size::Exact(32)), Style::Noop("rounded-full"), Style::Noop("ring-2"), Style::Noop("ring-white")]);
						props.set_attribute("src", "https://images.unsplash.com/photo-1500648767791-00dcc994a43e?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2.25&w=256&h=256&q=80");
						props.set_attribute("alt", "");
					});
					props.child("7", Dynamic).run("img", |mut props| {
						props.styles(&[Style::InlineBlock, Style::Width(Size::Exact(32)), Style::Width(Size::Exact(32)), Style::Noop("rounded-full"), Style::Noop("ring-2"), Style::Noop("ring-white")]);
						props.set_attribute("src", "https://images.unsplash.com/photo-1472099645785-5658abf4ff4e?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80");
						props.set_attribute("alt", "");
					});
				});
				props.child("5", Dynamic).run("div", |mut props| {
					props.styles(&[Style::Flex, Style::Noop("-space-x-2"), Style::Noop("overflow-hidden")]);
					
					props.child("1", Dynamic).run("img", |mut props| {
						props.styles(&[Style::InlineBlock, Style::Width(Size::Exact(40)), Style::Width(Size::Exact(40)), Style::Noop("rounded-full"), Style::Noop("ring-2"), Style::Noop("ring-white")]);
						props.set_attribute("src", "https://images.unsplash.com/photo-1491528323818-fdd1faba62cc?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80");
						props.set_attribute("alt", "");
					});
					props.child("3", Dynamic).run("img", |mut props| {
						props.styles(&[Style::InlineBlock, Style::Width(Size::Exact(40)), Style::Width(Size::Exact(40)), Style::Noop("rounded-full"), Style::Noop("ring-2"), Style::Noop("ring-white")]);
						props.set_attribute("src", "https://images.unsplash.com/photo-1550525811-e5869dd03032?ixlib=rb-1.2.1&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80");
						props.set_attribute("alt", "");
					});
					props.child("5", Dynamic).run("img", |mut props| {
						props.styles(&[Style::InlineBlock, Style::Width(Size::Exact(40)), Style::Width(Size::Exact(40)), Style::Noop("rounded-full"), Style::Noop("ring-2"), Style::Noop("ring-white")]);
						props.set_attribute("src", "https://images.unsplash.com/photo-1500648767791-00dcc994a43e?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2.25&w=256&h=256&q=80");
						props.set_attribute("alt", "");
					});
					props.child("7", Dynamic).run("img", |mut props| {
						props.styles(&[Style::InlineBlock, Style::Width(Size::Exact(40)), Style::Width(Size::Exact(40)), Style::Noop("rounded-full"), Style::Noop("ring-2"), Style::Noop("ring-white")]);
						props.set_attribute("src", "https://images.unsplash.com/photo-1472099645785-5658abf4ff4e?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80");
						props.set_attribute("alt", "");
					});
				});
			});
		});
	}
}

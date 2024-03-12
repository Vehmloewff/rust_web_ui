use rust_web_ui::*;

pub struct Example8;

pub struct Example8Props {}

impl Default for Example8Props {
	fn default() -> Example8Props {
		Example8Props { }
	}
}

impl Widget<'_> for Example8 {
	type Props = Example8Props;

	fn render(mut ctx: Ctx<'_>, props: Example8Props) {
		ctx.styles(&[NoStyle::Noop("")]);
		ctx.set_attribute("style", "");
		
		ctx.child("1", Dynamic).run("div", |props| {
			props.styles(&[Style::Color(Color::Fg(11)), Style::Padding(32)]);
			props.set_attribute("style", "min-height: 460px");
			
			props.child("1", Dynamic).run("div", |props| {
				props.styles(&[NoStyle::Noop("mx-auto"), Style::Width(256), NoStyle::Noop("text-right")]);
				
				props.child("1", Dynamic).run("div", |props| {
					props.set_attribute("x-data", "Components.menu({ open: true })");
					props.set_attribute("x-init", "init()");
					props.set_attribute("@keydown.escape.stop", "open = false; focusButton()");
					props.set_attribute("@click.away", "onClickAway($event)");
					props.styles(&[NoStyle::Noop("relative"), Style::InlineBlock, NoStyle::Noop("text-left")]);
					
					props.child("1", Dynamic).run("div", |props| {
						props.child("1", Dynamic).run("button", |props| {
							props.set_attribute("type", "button");
							props.styles(&[Style::InlineFlex, NoStyle::Noop("w-full"), Style::JustifyCenter, NoStyle::Noop("gap-x-1.5"), NoStyle::Noop("rounded-md"), NoStyle::Noop("bg-white"), Style::PaddingX(12), Style::PaddingY(8), NoStyle::Noop("text-sm"), Style::FontSemibold, Style::TextColor(Color::Fg(100)), NoStyle::Noop("shadow-sm"), NoStyle::Noop("ring-1"), NoStyle::Noop("ring-inset"), NoStyle::Noop("ring-gray-300"), Action::Hover(&[Style::Color(Color::Fg(6))])]);
							props.set_attribute("id", "menu-button");
							props.set_attribute("x-ref", "button");
							props.set_attribute("@click", "onButtonClick()");
							props.set_attribute("@keyup.space.prevent", "onButtonEnter()");
							props.set_attribute("@keydown.enter.prevent", "onButtonEnter()");
							props.set_attribute("aria-expanded", "true");
							props.set_attribute("aria-haspopup", "true");
							props.set_attribute("x-bind:aria-expanded", "open.toString()");
							props.set_attribute("@keydown.arrow-up.prevent", "onArrowUp()");
							props.set_attribute("@keydown.arrow-down.prevent", "onArrowDown()");
							
							props.child("0", Label).run(|props| props.set_text("Options"));
							props.child("1", Icon).run(|props| {
								props.style(&[NoStyle::Noop("-mr-1"), Style::Width(20), Style::Width(20), Style::TextColor(Color::Fg(44))]);
							});
						});
					});
					props.child("3", Dynamic).run("div", |props| {
						props.set_attribute("x-show", "open");
						props.set_attribute("x-transition:enter", "transition ease-out duration-100");
						props.set_attribute("x-transition:enter-start", "transform opacity-0 scale-95");
						props.set_attribute("x-transition:enter-end", "transform opacity-100 scale-100");
						props.set_attribute("x-transition:leave", "transition ease-in duration-75");
						props.set_attribute("x-transition:leave-start", "transform opacity-100 scale-100");
						props.set_attribute("x-transition:leave-end", "transform opacity-0 scale-95");
						props.styles(&[NoStyle::Noop("absolute"), NoStyle::Noop("right-0"), NoStyle::Noop("z-10"), Style::MarginTop(8), Style::Width(224), NoStyle::Noop("origin-top-right"), NoStyle::Noop("divide-y"), NoStyle::Noop("divide-gray-100"), NoStyle::Noop("rounded-md"), NoStyle::Noop("bg-white"), NoStyle::Noop("shadow-lg"), NoStyle::Noop("ring-1"), NoStyle::Noop("ring-black"), NoStyle::Noop("ring-opacity-5"), Action::Hover(&[NoStyle::Noop("outline-none")])]);
						props.set_attribute("x-ref", "menu-items");
						props.set_attribute("x-description", "Dropdown menu, show/hide based on menu state.");
						props.set_attribute("x-bind:aria-activedescendant", "activeDescendant");
						props.set_attribute("role", "menu");
						props.set_attribute("aria-orientation", "vertical");
						props.set_attribute("aria-labelledby", "menu-button");
						props.set_attribute("tabindex", "-1");
						props.set_attribute("@keydown.arrow-up.prevent", "onArrowUp()");
						props.set_attribute("@keydown.arrow-down.prevent", "onArrowDown()");
						props.set_attribute("@keydown.tab", "open = false");
						props.set_attribute("@keydown.enter.prevent", "open = false; focusButton()");
						props.set_attribute("@keyup.space.prevent", "open = false; focusButton()");
						
						props.child("1", Dynamic).run("div", |props| {
							props.styles(&[Style::PaddingY(4)]);
							props.set_attribute("role", "none");
							
							props.child("1", Dynamic).run("a", |props| {
								props.set_attribute("href", "#");
								props.styles(&[Style::TextColor(Color::Fg(78)), Style::Block, Style::PaddingX(16), Style::PaddingY(8), NoStyle::Noop("text-sm")]);
								props.set_attribute("x-state:on", "Active");
								props.set_attribute("x-state:off", "Not Active");
								props.set_attribute(":class", "{ 'bg-gray-100 text-gray-900': activeIndex === 0, 'text-gray-700': !(activeIndex === 0) }");
								props.set_attribute("role", "menuitem");
								props.set_attribute("tabindex", "-1");
								props.set_attribute("id", "menu-item-0");
								props.set_attribute("@mouseenter", "onMouseEnter($event)");
								props.set_attribute("@mousemove", "onMouseMove($event, 0)");
								props.set_attribute("@mouseleave", "onMouseLeave($event)");
								props.set_attribute("@click", "open = false; focusButton()");
								
								props.child("0", Label).run(|props| props.set_text("Edit"));
							});
							props.child("3", Dynamic).run("a", |props| {
								props.set_attribute("href", "#");
								props.styles(&[Style::TextColor(Color::Fg(78)), Style::Block, Style::PaddingX(16), Style::PaddingY(8), NoStyle::Noop("text-sm")]);
								props.set_attribute(":class", "{ 'bg-gray-100 text-gray-900': activeIndex === 1, 'text-gray-700': !(activeIndex === 1) }");
								props.set_attribute("role", "menuitem");
								props.set_attribute("tabindex", "-1");
								props.set_attribute("id", "menu-item-1");
								props.set_attribute("@mouseenter", "onMouseEnter($event)");
								props.set_attribute("@mousemove", "onMouseMove($event, 1)");
								props.set_attribute("@mouseleave", "onMouseLeave($event)");
								props.set_attribute("@click", "open = false; focusButton()");
								
								props.child("0", Label).run(|props| props.set_text("Duplicate"));
							});
						});
						props.child("3", Dynamic).run("div", |props| {
							props.styles(&[Style::PaddingY(4)]);
							props.set_attribute("role", "none");
							
							props.child("1", Dynamic).run("a", |props| {
								props.set_attribute("href", "#");
								props.styles(&[Style::TextColor(Color::Fg(78)), Style::Block, Style::PaddingX(16), Style::PaddingY(8), NoStyle::Noop("text-sm")]);
								props.set_attribute(":class", "{ 'bg-gray-100 text-gray-900': activeIndex === 2, 'text-gray-700': !(activeIndex === 2) }");
								props.set_attribute("role", "menuitem");
								props.set_attribute("tabindex", "-1");
								props.set_attribute("id", "menu-item-2");
								props.set_attribute("@mouseenter", "onMouseEnter($event)");
								props.set_attribute("@mousemove", "onMouseMove($event, 2)");
								props.set_attribute("@mouseleave", "onMouseLeave($event)");
								props.set_attribute("@click", "open = false; focusButton()");
								
								props.child("0", Label).run(|props| props.set_text("Archive"));
							});
							props.child("3", Dynamic).run("a", |props| {
								props.set_attribute("href", "#");
								props.styles(&[Style::TextColor(Color::Fg(78)), Style::Block, Style::PaddingX(16), Style::PaddingY(8), NoStyle::Noop("text-sm")]);
								props.set_attribute(":class", "{ 'bg-gray-100 text-gray-900': activeIndex === 3, 'text-gray-700': !(activeIndex === 3) }");
								props.set_attribute("role", "menuitem");
								props.set_attribute("tabindex", "-1");
								props.set_attribute("id", "menu-item-3");
								props.set_attribute("@mouseenter", "onMouseEnter($event)");
								props.set_attribute("@mousemove", "onMouseMove($event, 3)");
								props.set_attribute("@mouseleave", "onMouseLeave($event)");
								props.set_attribute("@click", "open = false; focusButton()");
								
								props.child("0", Label).run(|props| props.set_text("Move"));
							});
						});
						props.child("5", Dynamic).run("div", |props| {
							props.styles(&[Style::PaddingY(4)]);
							props.set_attribute("role", "none");
							
							props.child("1", Dynamic).run("a", |props| {
								props.set_attribute("href", "#");
								props.styles(&[Style::TextColor(Color::Fg(78)), Style::Block, Style::PaddingX(16), Style::PaddingY(8), NoStyle::Noop("text-sm")]);
								props.set_attribute(":class", "{ 'bg-gray-100 text-gray-900': activeIndex === 4, 'text-gray-700': !(activeIndex === 4) }");
								props.set_attribute("role", "menuitem");
								props.set_attribute("tabindex", "-1");
								props.set_attribute("id", "menu-item-4");
								props.set_attribute("@mouseenter", "onMouseEnter($event)");
								props.set_attribute("@mousemove", "onMouseMove($event, 4)");
								props.set_attribute("@mouseleave", "onMouseLeave($event)");
								props.set_attribute("@click", "open = false; focusButton()");
								
								props.child("0", Label).run(|props| props.set_text("Share"));
							});
							props.child("3", Dynamic).run("a", |props| {
								props.set_attribute("href", "#");
								props.styles(&[Style::TextColor(Color::Fg(78)), Style::Block, Style::PaddingX(16), Style::PaddingY(8), NoStyle::Noop("text-sm")]);
								props.set_attribute(":class", "{ 'bg-gray-100 text-gray-900': activeIndex === 5, 'text-gray-700': !(activeIndex === 5) }");
								props.set_attribute("role", "menuitem");
								props.set_attribute("tabindex", "-1");
								props.set_attribute("id", "menu-item-5");
								props.set_attribute("@mouseenter", "onMouseEnter($event)");
								props.set_attribute("@mousemove", "onMouseMove($event, 5)");
								props.set_attribute("@mouseleave", "onMouseLeave($event)");
								props.set_attribute("@click", "open = false; focusButton()");
								
								props.child("0", Label).run(|props| props.set_text("Add to favorites"));
							});
						});
						props.child("7", Dynamic).run("div", |props| {
							props.styles(&[Style::PaddingY(4)]);
							props.set_attribute("role", "none");
							
							props.child("1", Dynamic).run("a", |props| {
								props.set_attribute("href", "#");
								props.styles(&[Style::TextColor(Color::Fg(78)), Style::Block, Style::PaddingX(16), Style::PaddingY(8), NoStyle::Noop("text-sm")]);
								props.set_attribute(":class", "{ 'bg-gray-100 text-gray-900': activeIndex === 6, 'text-gray-700': !(activeIndex === 6) }");
								props.set_attribute("role", "menuitem");
								props.set_attribute("tabindex", "-1");
								props.set_attribute("id", "menu-item-6");
								props.set_attribute("@mouseenter", "onMouseEnter($event)");
								props.set_attribute("@mousemove", "onMouseMove($event, 6)");
								props.set_attribute("@mouseleave", "onMouseLeave($event)");
								props.set_attribute("@click", "open = false; focusButton()");
								
								props.child("0", Label).run(|props| props.set_text("Delete"));
							});
						});
					});
				});
			});
		});
	}
}
